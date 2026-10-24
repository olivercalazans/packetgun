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


use std::net::Ipv4Addr;
use crate::models::MAC;
use crate::packets::Checksum;



pub(crate) struct TcpPkt {
    buffer : [u8; 54],
}


impl TcpPkt {

    pub fn new() -> Self {
        let buffer = Self::build_fixed();
        Self { buffer }
    }



    fn build_fixed() -> [u8; 54] {
        let mut buffer = [0u8; 54];

        // Ethernet header (0 - 14)
        buffer[12..14].copy_from_slice(&0x0800u16.to_be_bytes());

        // IP header (14 - 34)
        buffer[14] = (4 << 4) | 5;
        buffer[15] = 0;
        buffer[16..18].copy_from_slice(&40u16.to_be_bytes());
        buffer[18..20].copy_from_slice(&0x1234u16.to_be_bytes());
        buffer[20..22].copy_from_slice(&0x4000u16.to_be_bytes());
        buffer[22] = 64;
        buffer[23] = 6;

        // TCP header (34 - 54)
        buffer[38..42].copy_from_slice(&1u32.to_be_bytes());
        buffer[42..46].copy_from_slice(&0u32.to_be_bytes());
        buffer[46] = 5 << 4;
        buffer[47] = 0x02;
        buffer[48..50].copy_from_slice(&64240u16.to_be_bytes());
        buffer[52..54].copy_from_slice(&0u16.to_be_bytes());

        buffer
    }



    #[inline]
    fn ether_header(
        &mut self,
        src_mac : MAC,
        dst_mac : MAC
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

        let cksum = Checksum::ipv4_checksum(&self.buffer[14..34]);
        self.buffer[24..26].copy_from_slice(&cksum.to_be_bytes());
    }



    #[inline]
    fn tcp_header(
        &mut self,
        src_ip   : Ipv4Addr,
        src_port : u16, 
        dst_ip   : Ipv4Addr,
        dst_port : u16,
    ) {
        self.buffer[34..36].copy_from_slice(&src_port.to_be_bytes());
        self.buffer[36..38].copy_from_slice(&dst_port.to_be_bytes());
        // 38..50 pre built
        self.buffer[50..52].copy_from_slice(&0u16.to_be_bytes());
        // 52..54 pre built

        let cksum = Checksum::tcp_checksum(&self.buffer[34..54], src_ip, dst_ip, 6);
        self.buffer[50..52].copy_from_slice(&cksum.to_be_bytes());
    }



    #[inline]
    pub fn pkt(
        &mut self,
        src_mac  : MAC,
        src_ip   : Ipv4Addr,
        src_port : u16,
        dst_mac  : MAC,
        dst_ip   : Ipv4Addr,
        dst_port : u16,
    ) 
      -> &[u8]
    {
        self.tcp_header(src_ip, src_port, dst_ip, dst_port);
        self.ip_header(src_ip, dst_ip);
        self.ether_header(src_mac, dst_mac);
        
        &self.buffer[..54]
    }
    
}