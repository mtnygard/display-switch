//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

#![windows_subsystem = "windows"]
#[macro_use]
extern crate log;
extern crate log_panics;

mod configuration;
mod display_control;
mod logging;
mod pnp_detect;
mod usb_devices;

fn main() {
    logging::init_logging().unwrap();
    let config = configuration::Configuration::load().unwrap();
    let mut detector = usb_devices::UsbChangeDetector::new().unwrap();
    let pnp_detect = pnp_detect::PnPDetect::new(|| {
        let added_devices = detector.detect_added_devices();
        match added_devices {
            Ok(devices) => {
                debug!("Detected device change. Added devices: {:?}", devices);
                if devices.contains(&config.usb_device) {
                    info!("Detected device we're looking for {:?}", &config.usb_device);
                    display_control::wiggle_mouse();
                    display_control::switch_to(&config.monitor_description, config.monitor_input)
                        .unwrap_or_else(|err| {
                            error!("Cannot switch monitor input: {:?}", err);
                        });
                }
            }
            Err(e) => {
                debug!("Cannot read devices: {:?}", e);
            }
        }
    });
    display_control::log_current_source(&config.monitor_description).unwrap_or_else(|err| {
        error!("Cannot get monitor input: {:?}", err);
    });
    pnp_detect.detect();
}
