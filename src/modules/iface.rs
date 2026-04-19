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

use std::{net::Ipv4Addr, ffi::CStr, fs , str::FromStr, fmt};
use libc::{freeifaddrs, AF_INET, sockaddr_in};
use crate::{SysInfo, abort, Mac};



#[derive(Clone, Default)]
pub struct Iface{
    iface: String,
}


impl Iface {

    pub fn new(iface_name: &str) -> Self {
        let iface = SysInfo::exists(iface_name)
            .unwrap_or_else(|e| abort(e));

        Self { iface }
    }



    pub fn index(&self) -> i32 {
        let ifindex_path = format!("/sys/class/net/{}/ifindex", &self.iface);
        
        match fs::read_to_string(&ifindex_path) {
            Ok(content) => {
                content.trim().parse().unwrap_or_else(|_| {
                    abort(&format!("Failed to parse ifindex for interface: {}", &self.iface));
                })
            }
            Err(_) => {
                abort(&format!("Uneable to read {}", ifindex_path));
            }
        }
    }



    pub fn name(&self) -> &str {
        &self.iface
    }



    pub fn mac(&self) -> Result<Mac, String> {
        let file_path = format!("/sys/class/net/{}/address", self.iface);
        
        let mac = fs::read_to_string(&file_path)
            .map(|content| content.trim().to_string())
            .map_err(|e| format!("Failed to read {}: {}", file_path, e))?;

        Mac::from_str(&mac)
    }



    pub fn cidr(&self) -> Result<String, String> {
        unsafe {
            let ifap    = SysInfo::ifaddrs_ptr();
            let mut cur = ifap;

            while !cur.is_null() {
                let ifa       = &*cur;
                let name_cstr = CStr::from_ptr(ifa.ifa_name);
                let name      = name_cstr.to_string_lossy();

                if name != self.iface
                    || ifa.ifa_addr.is_null()
                    || ifa.ifa_netmask.is_null()
                    || (*ifa.ifa_addr).sa_family as i32 != AF_INET
                {
                    cur = ifa.ifa_next;
                    continue;
                }

                let addr = &*(ifa.ifa_addr as *const sockaddr_in);
                let ip   = Ipv4Addr::from(addr.sin_addr.s_addr.to_ne_bytes());

                let netmask = &*(ifa.ifa_netmask as *const sockaddr_in);
                let mask    = Ipv4Addr::from(netmask.sin_addr.s_addr.to_ne_bytes());

                freeifaddrs(ifap);

                let cidr        = mask.octets().iter().map(|b| b.count_ones()).sum::<u32>() as u8;
                let ip_u32      = u32::from(ip);
                let mask_u32    = u32::from(mask);
                let network_u32 = ip_u32 & mask_u32;
                let network     = Ipv4Addr::from(network_u32.to_be_bytes());

                return Ok(format!("{}/{}", network, cidr));
            }

            freeifaddrs(ifap);
            Err(format!("Interface {} not found or missing IPv4/netmask", &self.iface))
        }
    }



    pub fn gateway_mac(&self) -> Result<Mac, String> {
        let arp_content = fs::read_to_string("/proc/net/arp")
            .map_err(|e| format!("Uneable to read /proc/net/arp: {}", e))?;

        for line in arp_content.lines().skip(1) {
            let fields: Vec<&str> = line.split_whitespace().collect();

            if fields.len() < 6 || fields[5] != &self.iface {
                continue;
            }

            let mac = fields[3];
            if mac.is_empty() || mac == "00:00:00:00:00:00" {
                continue
            }
            
            return Mac::from_str(mac);
        }

        Err(format!("No MAC address found on interface {}", &self.iface))
    }

}



impl FromStr for Iface {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Interface name cannot be empty".to_string());
        }
        Ok(Iface { iface: s.to_string() })
    }
}


impl fmt::Display for Iface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

 
impl fmt::Debug for Iface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Iface{{iface: {}}}", self.name())
    }
}