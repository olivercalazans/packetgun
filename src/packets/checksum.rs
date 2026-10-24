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


pub(super) struct Checksum;


impl Checksum {

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

    

    pub fn tcp_checksum(
        header   : &[u8],
        src_ip   : Ipv4Addr,
        dst_ip   : Ipv4Addr,
        protocol : u8,
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
        
        sum += protocol as u32;
        sum += header.len() as u32;
        
        Self::calculate_checksum(sum, header)
    }

    

    pub fn icmp_checksum(header: &[u8]) -> u16 {
        Self::calculate_checksum(0, header)
    }
    


    pub fn ipv4_checksum(header: &[u8]) -> u16 {
        Self::calculate_checksum(0, header)
    }
    
}