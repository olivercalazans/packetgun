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



pub(crate) struct IcmpPkt {
    buffer: [u8; 42],
}


impl IcmpPkt {

    pub fn new() -> Self {
        let buffer = Self::build_fixed();
        Self { buffer }
    }



    fn build_fixed() -> [u8; 42] {
        let mut buffer = [0u8; 42];

        // Ethernet header (0 - 14)
        buffer[12..14].copy_from_slice(&0x0800u16.to_be_bytes());

        // IP header (14 - 34)
        buffer[14] = (4 << 4) | 5;
        buffer[15] = 0;
        buffer[16..18].copy_from_slice(&28u16.to_be_bytes());
        buffer[18..20].copy_from_slice(&0x1234u16.to_be_bytes());
        buffer[20..22].copy_from_slice(&0x4000u16.to_be_bytes());
        buffer[22] = 64;
        buffer[23] = 1;
        buffer[24..26].copy_from_slice(&0u16.to_be_bytes());

        // ICMP header (34 - 42)
        buffer[34] = 8;
        buffer[35] = 0;
        buffer[36..38].copy_from_slice(&0u16.to_be_bytes());
        buffer[38..40].copy_from_slice(&0x1234u16.to_be_bytes()); 
        buffer[40..42].copy_from_slice(&1u16.to_be_bytes());

        let cksum = Checksum::icmp_checksum(&buffer[34..42]);
        buffer[36..38].copy_from_slice(&cksum.to_be_bytes());

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
        // 14..26 pre built
        self.buffer[26..30].copy_from_slice(&src_ip.octets());
        self.buffer[30..34].copy_from_slice(&dst_ip.octets());

        let cksum = Checksum::ipv4_checksum(&self.buffer[14..34]);
        self.buffer[24..26].copy_from_slice(&cksum.to_be_bytes());
    }



    #[inline]
    pub fn l2_pkt(
        &mut self,
        src_mac : MAC,
        src_ip  : Ipv4Addr,
        dst_mac : MAC,
        dst_ip  : Ipv4Addr,
    ) 
      -> &[u8]
    {
        self.ip_header(src_ip, dst_ip);
        self.ether_header(src_mac, dst_mac);

        &self.buffer[..42]
    }

}