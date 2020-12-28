//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//
#[macro_use]
extern crate log;
extern crate libc;
extern crate log_panics;
extern crate udev;

use std::io;
use std::ptr;
use std::thread;
use std::time::Duration;

use std::os::unix::io::AsRawFd;

use libc::{c_int, c_short, c_ulong, c_void};

mod configuration;
mod display_control;

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

#[repr(C)]
struct sigset_t {
    __private: c_void,
}

#[allow(non_camel_case_types)]
type nfds_t = c_ulong;

const POLLIN: c_short = 0x0001;

extern "C" {
    fn ppoll(
        fds: *mut pollfd,
        nfds: nfds_t,
        timeout_ts: *mut libc::timespec,
        sigmask: *const sigset_t,
    ) -> c_int;
}

fn main() -> io::Result<()> {
    env_logger::init();

    let config = configuration::Configuration::load().unwrap();

    let mut socket = udev::MonitorBuilder::new()?
        .match_subsystem_devtype("usb", "usb_device")?
        .listen()?;

    let mut fds = vec![pollfd {
        fd: socket.as_raw_fd(),
        events: POLLIN,
        revents: 0,
    }];

    loop {
        let result = unsafe {
            ppoll(
                (&mut fds[..]).as_mut_ptr(),
                fds.len() as nfds_t,
                ptr::null_mut(),
                ptr::null(),
            )
        };

        if result < 0 {
            return Err(io::Error::last_os_error());
        }

        let event = match socket.next() {
            Some(evt) => evt,
            None => {
                thread::sleep(Duration::from_millis(10));
                continue;
            }
        };

        debug!(
            "{}: {} {} (subsystem={}, sysname={}, devtype={})",
            event.sequence_number(),
            event.event_type(),
            event.syspath().to_str().unwrap_or("---"),
            event
                .subsystem()
                .map_or("", |s| { s.to_str().unwrap_or("") }),
            event.sysname().to_str().unwrap_or(""),
            event.devtype().map_or("", |s| { s.to_str().unwrap_or("") })
        );

        if event.event_type() == udev::EventType::Bind {
            for _ in event
                .attributes()
                .map(|attr| (attr.name().to_str(), attr.value()))
                .filter(|(name, _)| match name {
                    Some(name) => name == &"product",
                    None => false,
                })
                .filter(|(_, value)| match value {
                    Some(value) => value == &config.usb_product.as_str(),
                    None => false,
                })
            {
                info!("Detected {:?}", &config.usb_product);
                match display_control::switch_to(
                    &config.monitor_description,
                    display_control::InputSource::DisplayPort1,
                ) {
                    Ok(_) => info!(
                        "Switched {:?} to {:?}",
                        &config.monitor_description, &config.monitor_input
                    ),
                    Err(e) => error!("Error switching display: {}", e),
                }
            }
        }
    }
}
