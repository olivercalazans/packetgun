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


use std::num::NonZeroU32;
use std::ptr;
use xsk_rs::{
    config::{UmemConfig, SocketConfig, Interface, BindFlags, XdpFlags},
    umem::frame::FrameDesc,
    Socket, Umem,
};
use crate::{Iface, abort};



pub struct Layer2Socket {
    tx_q  : xsk_rs::TxQueue,
    _umem : Umem,
    descs : Vec<FrameDesc>,
}


impl Layer2Socket {
    pub fn new(iface: &Iface) -> Self {
        let frame_count = 4096;
        let umem_config = UmemConfig::default();
        
        let (umem, descs) = Umem::new(
                umem_config,
                NonZeroU32::new(frame_count).unwrap(),
                false,
            ).unwrap_or_else(|e| abort(&format!("UMEM Error: {}", e)));

        let name: Interface = iface.name().parse()
            .unwrap_or_else(|_| abort(&format!("Invalid Interface: {}", iface.name())));

        let socket_config = SocketConfig::builder()
            .bind_flags(BindFlags::XDP_COPY | BindFlags::XDP_USE_NEED_WAKEUP)
            .xdp_flags(XdpFlags::XDP_FLAGS_SKB_MODE) 
            .build();

        let (tx_q, _rx_q, _fq_cq) = unsafe {
            Socket::new(
                socket_config,
                &umem,
                &name,
                0,
            ).unwrap_or_else(|e| abort(&format!("Socket Error: {}", e)))
        };

        Self { tx_q, _umem: umem, descs }
    }



    #[inline]
    pub fn send(&mut self, frame: &[u8]) {
        let len = frame.len();

        if let Some(mut desc) = self.descs.pop() {
            unsafe {
                let mut data_mut = self._umem.data_mut(&mut desc);
                
                ptr::copy_nonoverlapping(
                    frame.as_ptr(),
                    data_mut.as_mut_ptr(),
                    len,
                );

                let slice = [desc];

                if self.tx_q.produce(&slice) > 0 {
                    let _ = self.tx_q.wakeup();
                }
            }

            self.descs.push(desc);
        }
    }
}
