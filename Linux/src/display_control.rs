// Original version Copyright © 2020 Haim Gelfenbeyn
// display_control.rs module rewritten using code from Copyright 2018 Maxwell Koo <mjkoo90@gmail.com>
// Derived from ddcset, Copyright (c) 2018 arcnmx
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use conv::TryFrom;
use ddc_hi::{Backend, Ddc, DdcHost, Display, Query};
use enum_derive::*;
use failure::{format_err, Error};
use macro_attr::*;
use serde::Deserialize;

/// VCP feature code for input select
const INPUT_SELECT: u8 = 0x60;

macro_attr! {
    #[derive(Deserialize, Clone, Copy, Debug, PartialEq, EnumDisplay!, EnumFromStr!, IterVariantNames!(InputSourceVariantNames), TryFrom!(u16))]
    #[repr(u8)]
    pub enum InputSource {
        Vga1 = 0x01,
        Vga2 = 0x02,
        Dvi1 = 0x03,
        Dvi2 = 0x04,
        CompositeVideo1 = 0x05,
        CompositeVideo2 = 0x06,
        SVideo1 = 0x07,
        SVideo2 = 0x08,
        Tuner1 = 0x09,
        Tuner2 = 0x0a,
        Tuner3 = 0x0b,
        ComponentVideo1 = 0x0c,
        ComponentVideo2 = 0x0d,
        ComponentVideo3 = 0x0e,
        DisplayPort1 = 0x0f,
        DisplayPort2 = 0x10,
        Hdmi1 = 0x11,
        Hdmi2 = 0x12,
    }
}
/// Tracks set of displays to wait for when dropped
#[derive(Default)]
struct DisplaySleep(Vec<Display>);

impl DisplaySleep {
    /// Add a display to the tracked set
    fn add(&mut self, display: Display) {
        self.0.push(display)
    }
}

impl Drop for DisplaySleep {
    /// Wait for display communication delays before exiting
    fn drop(&mut self) {
        info!("Waiting for display communication delays before exit");
        for display in &mut self.0 {
            display.handle.sleep()
        }
    }
}

/// Return all known display handles matching a given query
fn displays(query: (Query, bool)) -> Result<Vec<Display>, Error> {
    let needs_caps = query.1;
    let query = query.0;
    Display::enumerate()
        .into_iter()
        .map(|mut d| {
            if needs_caps && d.info.backend == Backend::WinApi {
                d.update_capabilities().map(|_| d)
            } else {
                Ok(d)
            }
        })
        .filter(|d| {
            if let Ok(ref d) = *d {
                query.matches(&d.info)
            } else {
                true
            }
        })
        .collect()
}

/// Set the input source of a display to a given input
fn set_input_source(display: &mut Display, input_source: InputSource) -> Result<(), Error> {
    if let Some(feature) = display.info.mccs_database.get(INPUT_SELECT) {
        display
            .handle
            .set_vcp_feature(feature.code, input_source as u16)
    } else {
        Err(format_err!("Could not access input source feature"))
    }
}

/// Get the current input source of a display
// fn get_input_source(display: &mut Display) -> Result<InputSource, Error> {
//     if let Some(feature) = display.info.mccs_database.get(INPUT_SELECT) {
//         InputSource::try_from(display.handle.get_vcp_feature(feature.code)?.value())
//             .map_err(Error::from)
//     } else {
//         Err(format_err!("Could not access input source feature"))
//     }
// }

pub fn switch_to(model: &String, input_source: InputSource) -> Result<(), Error> {
    let query = (Query::ModelName(model.into()), true);

    let mut sleep = DisplaySleep::default();

    for mut display in displays(query)? {
        display.update_capabilities()?;
        // This sometimes fails but the switch still succeeded, ignore the Err for now
        if let Err(e) = set_input_source(&mut display, input_source) {
            warn!("Error while setting input: {}", e)
        }
        sleep.add(display);
    }
    Ok(())
}
