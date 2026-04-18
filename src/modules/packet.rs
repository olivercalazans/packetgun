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
use crate::{Mac, abort};



pub struct Packet {
    buffer: [u8; 54],
}


impl Packet {

    pub fn new(proto: u8) -> Self {
        let buffer = Self::build_fixed(proto);
        Self { buffer }
    }



    fn build_fixed(proto: u8) -> [u8; 54] {
        let mut buffer = [0u8; 54];

        // Ethernet header (0 - 14)
        buffer[12..14].copy_from_slice(&0x0800u16.to_be_bytes());

        // IP header (14 - 34)
        buffer[14] = (4 << 4) | 5;
        buffer[15] = 0;
        
        let total_len:u16 = match proto {
            1 => 28,
            6 => 40,
            _ => abort(format!("Unknown protocol: {}", proto)),
        };
        
        buffer[16..18].copy_from_slice(&total_len.to_be_bytes());
        buffer[18..20].copy_from_slice(&0x1234u16.to_be_bytes());
        buffer[20..22].copy_from_slice(&0x4000u16.to_be_bytes());
        buffer[22] = 64;
        buffer[23] = proto;
        buffer[24..26].copy_from_slice(&0u16.to_be_bytes());

        match proto {
            1 => Self::build_icmp_fixed(&mut buffer),
            6 => Self::build_tcp_fixed(&mut buffer),
            _ => abort(format!("Unknown protocol: {}", proto)),
        }

        buffer
    }


    
    fn build_icmp_fixed(buffer: &mut [u8; 54]) {
        // ICMP header (34 - 42)
        buffer[34] = 8;
        buffer[35] = 0;
        buffer[36..38].copy_from_slice(&0u16.to_be_bytes());
        buffer[38..40].copy_from_slice(&0x1234u16.to_be_bytes()); 
        buffer[40..42].copy_from_slice(&1u16.to_be_bytes());

        let cksum = Self::icmp_checksum(&buffer[34..42]);
        buffer[36..38].copy_from_slice(&cksum.to_be_bytes());
    }



    fn build_tcp_fixed(buffer: &mut [u8; 54]) {
        // TCP header (34 - 54)
        buffer[38..42].copy_from_slice(&1u32.to_be_bytes());
        buffer[42..46].copy_from_slice(&0u32.to_be_bytes());
        buffer[46] = 5 << 4;
        buffer[47] = 0x02;
        buffer[48..50].copy_from_slice(&64240u16.to_be_bytes());
        buffer[52..54].copy_from_slice(&0u16.to_be_bytes());
    }



    #[inline]
    fn ether_header(
        &mut self,
        src_mac : Mac,
        dst_mac : Mac
    ) {
        self.buffer[..6].copy_from_slice(dst_mac.bytes());
        self.buffer[6..12].copy_from_slice(src_mac.bytes());
        // 12..14 pre built
    }



    #[inline]
    fn ip_header(
        &mut self,
        src_ip : Ipv4Addr,
        dst_ip : Ipv4Addr
    ) {
        // 14..24 pre built
        self.buffer[24..26].copy_from_slice(&0u16.to_be_bytes());
        self.buffer[26..30].copy_from_slice(&src_ip.octets());
        self.buffer[30..34].copy_from_slice(&dst_ip.octets());

        let cksum = Self::ipv4_checksum(&self.buffer[14..34]);
        self.buffer[24..26].copy_from_slice(&cksum.to_be_bytes());
    }




    #[inline]
    pub fn icmp_pkt(
        &mut self,
        src_mac : Mac,
        src_ip  : Ipv4Addr,
        dst_mac : Mac,
        dst_ip  : Ipv4Addr,
    ) 
      -> &[u8]
    {
        self.ip_header(src_ip, dst_ip);
        self.ether_header(src_mac, dst_mac);

        &self.buffer[..42]
    }



    #[inline]
    pub fn tcp_pkt(
        &mut self,
        src_mac  : Mac,
        src_ip   : Ipv4Addr,
        src_port : u16,
        dst_mac  : Mac,
        dst_ip   : Ipv4Addr,
        dst_port : u16,
    ) 
      -> &[u8]
    {
        self.buffer[34..36].copy_from_slice(&src_port.to_be_bytes());
        self.buffer[36..38].copy_from_slice(&dst_port.to_be_bytes());
        // 38..50 pre built
        self.buffer[50..52].copy_from_slice(&0u16.to_be_bytes());
        // 52..54 pre built

        let cksum = Self::tcp_checksum(&self.buffer[34..54], src_ip, dst_ip);
        self.buffer[50..52].copy_from_slice(&cksum.to_be_bytes());
        
        self.ip_header(src_ip, dst_ip);
        self.ether_header(src_mac, dst_mac);
        
        &self.buffer[..54]
    }



    fn calculate_checksum(mut sum: u32, data: &[u8]) -> u16 {
        let mut i = 0;
        
        while i + 1 < data.len() {
            sum += ((data[i] as u32) << 8) | data[i + 1] as u32;
            i += 2;
        }
        
        if i < data.len() {
            sum += (data[i] as u32) << 8;
        }
        
        while (sum >> 16) != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        
        !(sum as u16)
    }

    

    fn tcp_checksum(
        header : &[u8],
        src_ip : Ipv4Addr,
        dst_ip : Ipv4Addr,
    ) 
      -> u16
    {
        let mut sum    = 0u32;
        let src_octets = src_ip.octets();
        let dst_octets = dst_ip.octets();
        
        sum += ((src_octets[0] as u32) << 8 | src_octets[1] as u32)
             + ((src_octets[2] as u32) << 8 | src_octets[3] as u32);
        
        sum += ((dst_octets[0] as u32) << 8 | dst_octets[1] as u32)
             + ((dst_octets[2] as u32) << 8 | dst_octets[3] as u32);
        
        sum += 6u32;
        sum += header.len() as u32;
        
        Self::calculate_checksum(sum, header)
    }

    

    fn icmp_checksum(header: &[u8]) -> u16 {
        Self::calculate_checksum(0, header)
    }
    


    fn ipv4_checksum(header: &[u8]) -> u16 {
        Self::calculate_checksum(0, header)
    }

}