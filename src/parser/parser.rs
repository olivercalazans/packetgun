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
use clap::Parser;



#[derive(Parser)]
#[command(name = "pktgun", about = "PacketGun")]
pub struct Args {

    /// Interface
    #[arg(short = 'i', long = "iface")]
    pub iface: String,


    /// Destination IP
    #[arg(long = "dip")]
    pub dst_ip: Ipv4Addr,


    /// Destination MAC
    #[arg(long = "dmac")]
    pub dst_mac: String,


    /// Target port
    #[arg(short, long)]
    pub port: u16,


    /// Optional source IP
    #[arg(long = "sip")]
    pub src_ip: Option<Ipv4Addr>,


    /// Optional source MAC
    #[arg(long = "smac")]
    pub src_mac: Option<String>,

}