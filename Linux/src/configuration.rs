//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use config::ConfigError;
use dirs;
use serde::Deserialize;

use crate::display_control;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub usb_product: String,
    pub monitor_input: display_control::InputSource,
    pub monitor_description: String,
}

fn config_file_name() -> Option<std::path::PathBuf> {
    dirs::config_dir().map(|d| d.join("display-switch").join("display-switch.ini"))
}

impl Configuration {
    pub fn load() -> Result<Self, ConfigError> {
        let config_file_name = config_file_name().unwrap();
        let mut settings = config::Config::default();
        settings
            .merge(config::File::from(config_file_name.clone()))?
            .merge(config::Environment::with_prefix("DISPLAY_SWITCH"))?;
        let config = settings.try_into::<Self>()?;
        info!(
            "Configuration loaded ({:?}): {:?}",
            config_file_name, config
        );
        Ok(config)
    }
}
