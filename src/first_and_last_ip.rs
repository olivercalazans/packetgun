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
use crate::abort;


pub(crate) fn get_first_and_last_ip(cidr: &str) -> (u32, u32) {
    let parts: Vec<&str> = cidr.split('/').collect();

    if parts.len() != 2 {
        abort(&format!("Invalid CIDR: {}", cidr));
    }

    let ip_str     = parts[0];
    let prefix_str = parts[1];

    let ip: Ipv4Addr = ip_str.parse().unwrap_or_else(|e| {
        abort(&format!("Invalid IP in CIDR '{}': {}", cidr, e));
    });

    let prefix: u8 = prefix_str.parse().unwrap_or_else(|e| {
        abort(&format!("Invalid prefix in CIDR '{}': {}", cidr, e));
    });

    if prefix > 32 {
        abort(&format!("Prefix out of range in CIDR '{}': {}", cidr, prefix));
    }

    let ip_u32 = u32::from_be_bytes(ip.octets());

    let mask = if prefix == 0 {
        0u32
    } else {
        (!0u32).checked_shl(32 - prefix as u32).unwrap_or(0)
    };

    let network   = ip_u32 & mask;
    let broadcast = network | !mask;

    let usable_start = if network + 1 <= broadcast - 1 {
        network + 1
    } else {
        network
    };

    let usable_end = if network + 1 <= broadcast - 1 {
        broadcast - 1
    } else {
        network
    };

    (usable_start, usable_end)
}