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


mod abort;
pub use abort::abort;

mod arg_parser;
pub use arg_parser::Args;

mod first_and_last_ip;
pub use first_and_last_ip::get_first_and_last_ip;

mod iface;
pub use iface::Iface;

mod l2_socket;
pub use l2_socket::Layer2Socket;

mod mac_resolver;
pub use mac_resolver::resolve_mac;

mod mac;
pub use mac::Mac;

mod packet;
pub use packet::Packet;

mod rand_values;
pub use rand_values::RandomValues;

mod sys_info;
pub use sys_info::SysInfo;