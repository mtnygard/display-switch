//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

#![windows_subsystem = "windows"]
#[macro_use]
extern crate log;

mod configuration;
mod display_control;
mod logging;
mod pnp_detect;
mod usb_devices;

fn main() {
    logging::init_logging().unwrap();
    let config = configuration::Configuration::load().unwrap();
    let usb_device = config.usb_device.clone();
    let monitor_description = config.monitor_description.clone();
    let monitor_input = config.monitor_input.clone();
    let mut detector = usb_devices::UsbChangeDetector::new().unwrap();
    let pnp_detect = pnp_detect::PnPDetect::new(move || {
        let added_devices = detector.detect_added_devices().unwrap();
        debug!("Detected device change. Added devices: {:?}", added_devices);
        if added_devices.contains(&usb_device) {
            info!("Detected device we're looking for {:?}", &usb_device);
            display_control::wiggle_mouse();
            display_control::switch_to(&monitor_description, monitor_input).unwrap_or_else(|err| {
                error!("Cannot switch monitor input: {:?}", err);
            });
        }
    });
    display_control::log_current_source(&config.monitor_description).unwrap_or_else(|err| {
        error!("Cannot get monitor input: {:?}", err);
    });
    pnp_detect.detect();
}
