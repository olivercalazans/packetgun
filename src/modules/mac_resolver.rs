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


use crate::Iface;
use crate::{abort, Mac};



pub fn resolve_mac(
    input_mac : Option<String>, 
    iface     : &Iface
) 
  -> Option<Mac>
{
    if input_mac.is_none() {
        return None;
    }

    let mac = input_mac.unwrap();

    let mac = match mac.as_str() {
        "gateway" => iface.gateway_mac().unwrap_or_else(|e| abort(e)),
        "local"   => iface.mac().unwrap_or_else(|e| abort(e)),
        _         => Mac::from_str(&mac).unwrap_or_else(|e| abort(e))
    };
    
    Some(mac)
}