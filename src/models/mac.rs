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


use std::fmt;
use crate::utils::abort;


#[derive(Clone, Copy)]
pub struct MAC {
    bytes: [u8; 6],
}


impl MAC {

    pub fn new(bytes: [u8; 6]) -> Self {
        Self { bytes }
    }
    


    pub fn from_str(mac: &str) -> Self {
        let parts: Vec<&str> = mac.split(':').collect();

        if parts.len() != 6 {
            abort(&format!("Invalid address: {}", mac));
        }

        let mut bytes = [0u8; 6];
        
        for (i, part) in parts.iter().enumerate() {
            let byte = u8::from_str_radix(part, 16);

            if byte.is_err() {
                abort(&format!("Invalid MAC byte ({}): {}", part, byte.unwrap_err()))
            }

            bytes[i] = byte.unwrap()
        }

        Self { bytes }
    }



    pub fn bytes(&self) -> &[u8; 6] {
        &self.bytes
    }
}



impl fmt::Display for MAC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.bytes[0], self.bytes[1], self.bytes[2],
            self.bytes[3], self.bytes[4], self.bytes[5]
        )
    }
}