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
use clap::Parser;



#[derive(Parser)]
#[command(name = "pgun", about = "Packet Gun")]
pub struct Args {

    /// Use ICMP (Ping) packets
    #[arg(long = "icmp")]
    pub icmp: bool,

    
    /// Use TCP SYN packets
    #[arg(long = "tcp")]
    pub tcp: bool,


    /// Target IP address to flood
    #[arg(long = "dip")]
    pub dst_ip: Ipv4Addr,


    /// Use "local" = iface MAC, "gateway" = gateway MAC
    #[arg(long = "dmac")]
    pub dst_mac: String,


    /// Target port
    #[arg(short, long, default_value_t = 0)]
    pub port: u16,


    /// Optional source IP address
    #[arg(long = "sip")]
    pub src_ip: Option<Ipv4Addr>,


    /// Use "local" = iface MAC or "gateway" = gateway MAC
    #[arg(long = "smac")]
    pub src_mac: Option<String>,

}