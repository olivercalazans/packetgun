/*
 * Copyright (C) 2026 Oliver R. Calazans Jeronimo
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org>.
 */


use std::net::Ipv4Addr;
use std::time::Instant;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use clap::Parser;

mod modules;
use modules::*;



fn main() {
    let args        = Args::parse();
    let mut pkt_gun = PacketGun::new(&args);
    pkt_gun.execute();
}



pub struct PacketGun {
    builder  : Packet,
    iface    : Iface,
    rand     : RandomValues,
    protocol : u8,
    src_ip   : Option<Ipv4Addr>,
    src_mac  : Option<Mac>,
    dst_ip   : Ipv4Addr,
    dst_mac  : Mac,
    dst_port : u16,
    duration : f64,
    pkt_sent : usize,
}




impl PacketGun {

    pub fn new(args: &Args) -> Self {
        let iface = SysInfo::iface_from_ip(args.dst_ip);
        let cidr  = iface.cidr().unwrap_or_else(|e| abort(e));
        
        let (protocol, dst_port) = Self::validate_protocol(&args);
        let (first_ip, last_ip)  = get_first_and_last_ip(&cidr);

        Self {
            pkt_sent : 0,
            duration : 0.0,
            builder  : Packet::new(protocol),
            rand     : RandomValues::new(first_ip, last_ip),
            src_ip   : args.src_ip,
            src_mac  : resolve_mac(args.src_mac.clone(), &iface),
            dst_ip   : args.dst_ip,
            dst_mac  : resolve_mac(Some(args.dst_mac.clone()), &iface).unwrap(),
            dst_port,
            protocol,
            iface,
        }
    }



    fn validate_protocol(args: &Args) -> (u8, u16) {
        let proto = if args.proto != 1 && args.proto != 6 {
            abort("It is necessary to select a protocol. 1 for ICMP or 6 for TCP");
        } else {
            args.proto
        };

        let dst_port = if proto == 6 && args.dst_port == 0 {
            abort("A port must be set to use TCP packets. 0 is reserved");
        } else { 
            args.dst_port
        };

        (proto, dst_port)
    }

    
    
    pub fn execute(&mut self){
        self.display_info();
        self.send_endlessly();
        self.display_exec_info();
    }


    
    fn display_info(&self) {
        let src_mac = match self.src_mac {
            Some(mac) => mac.to_string(),
            None      => "Random".to_string(),
        };

        let src_ip = match self.src_ip {
            Some(ip) => ip.to_string(),
            None     => "Random".to_string(),
        };

        let dst_mac = self.dst_mac.to_string();

        println!("[*] SRC >> MAC: {} / IP: {}", src_mac, src_ip);
        println!("[*] DST >> MAC: {} / IP: {}", dst_mac, self.dst_ip);
        println!("[*] IFACE: {}", self.iface.name());
    }



    fn send_endlessly(&mut self) {
        let mut socket = Layer2Socket::new(&self.iface);

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            println!("\n[-] Flood interrupted");
            r.store(false, Ordering::SeqCst);
        }).expect("Error configuring Ctrl+C handler");

        println!("[+] Sending packets. Press CTRL + C to stop");
        let init = Instant::now();

        while running.load(Ordering::SeqCst) {
            let pkt = self.get_packet();
            socket.send(pkt);
            self.pkt_sent += 1;
        }

        self.duration = init.elapsed().as_secs_f64(); 
    }



    #[inline]
    fn get_packet(&mut self) -> &[u8] {
        match self.protocol {
            1 => self.get_icmp_pkt(),
            6 => self.get_tcp_pkt(),
            _ => abort(format!("Unknown protocol: {}", self.protocol)),
        }
    }



    #[inline]
    fn get_icmp_pkt(&mut self) -> &[u8] {
        self.builder.icmp_pkt(
            self.src_mac.unwrap_or_else(|| self.rand.random_mac()),
            self.src_ip.unwrap_or_else( || self.rand.random_ip()),
            self.dst_mac,
            self.dst_ip
        )
    }



    #[inline]
    fn get_tcp_pkt(&mut self) -> &[u8] {
        self.builder.tcp_pkt(
            self.src_mac.unwrap_or_else(|| self.rand.random_mac()), 
            self.src_ip.unwrap_or_else( || self.rand.random_ip()), 
            self.rand.random_port(),
            self.dst_mac, 
            self.dst_ip,
            self.dst_port,
        )
    }



    fn display_exec_info(&self) {
        println!("[%] {} packets sent in {:.2} seconds", &self.pkt_sent, self.duration);

        if self.duration > 1.0 {
            let rate = self.pkt_sent as f64 / self.duration;
            println!("[%] {:.2} packets sent per second", rate);
        };        
    }

}