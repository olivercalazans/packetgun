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

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use libc;



pub(crate) struct CtrlCHandler;


impl CtrlCHandler {

    pub fn setup(running: Arc<AtomicBool>) {
        unsafe {
            let mut mask: libc::sigset_t = std::mem::zeroed();
            libc::sigemptyset(&mut mask);
            libc::sigaddset(&mut mask, libc::SIGINT);
            
            libc::pthread_sigmask(libc::SIG_BLOCK, &mask, std::ptr::null_mut());
            
            let fd = libc::signalfd(-1, &mask, 0);
            if fd < 0 {
                eprintln!("Warning: Failed to create signalfd, Ctrl+C might not work properly");
                return;
            }
            
            thread::spawn(move || {
                Self::signal_loop(fd, running);
            });
        }
    }



    fn signal_loop(fd: i32, running: Arc<AtomicBool>) {
        unsafe {
            let mut fds = libc::pollfd {
                fd,
                events: libc::POLLIN,
                revents: 0,
            };
            
            while libc::poll(&mut fds, 1, -1) > 0 {
                if fds.revents & libc::POLLIN == 0 {
                    continue;
                }

                if Self::is_sigint_signal(fd) {
                    running.store(false, Ordering::SeqCst);
                    break;
                }
            }

            libc::close(fd);
        }
    }



    fn is_sigint_signal(fd: i32) -> bool {
        unsafe {
            let mut info: libc::signalfd_siginfo = std::mem::zeroed();
            let size = std::mem::size_of::<libc::signalfd_siginfo>();
            
            if libc::read(fd, &mut info as *mut _ as *mut libc::c_void, size) != size as isize {
                return false;
            }

            info.ssi_signo == libc::SIGINT as u32
        }
    }

}