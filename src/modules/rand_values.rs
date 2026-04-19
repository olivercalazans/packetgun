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
use rand::{Rng, rngs::ThreadRng};
use crate::Mac;



pub struct RandomValues {
    rng      : ThreadRng,
    first_ip : u32,
    last_ip  : u32,
}


impl RandomValues {

    pub fn new(first_ip: u32, last_ip: u32) -> Self {
        Self { 
            rng: rand::thread_rng(),
            first_ip,
            last_ip,
        }
    }



    #[inline]
    pub fn random_port(&mut self) -> u16 {
        self.rng.gen_range(49152..=65535)
    }



    #[inline]
    pub fn random_ip(&mut self) -> Ipv4Addr {
        let rand_num     = self.rng.gen_range(self.first_ip..=self.last_ip);
        let ip: Ipv4Addr = rand_num.into();
        ip
    }



    #[inline]
    fn random_vec_u8_6(&mut self) -> [u8; 6] {
        let mut bytes = [0u8; 6];
        for b in bytes.iter_mut() { *b = self.rng.r#gen(); }
        bytes[0] = (bytes[0] | 0x02) & 0xFE;
        bytes
    }



    #[inline]
    pub fn random_mac(&mut self) -> Mac {
        let bytes = self.random_vec_u8_6();
        Mac::new(bytes)
    }

}
