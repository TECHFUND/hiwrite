use std::fs;
use std::fs::File;
use serde::Deserialize;
use crate::plugin::error::PluginErrorCodes;
use crate::plugin::{Data, Plugin};

#[derive(Debug)]
#[repr(C)]
pub struct PluginMetadata {
    pub description: Option<String>,
    pub version: String,
    pub name: String,
    pub filename: String,
    pub objfile: String,
}

impl PluginMetadata {
    pub fn read_from_str<T: for<'a> Deserialize<'a>>(string: &str) -> Result<T, PluginErrorCodes> {
        let data: T = match toml::from_str(string) {
            Ok(t) => t,
            Err(e) => {
                log::error!("Couldn't read metadata file: {}", e.to_string());
                return Err(PluginErrorCodes::ParametersError);
            }
        };

        Ok(data)
    }

    pub(crate) fn load(plugin: &Plugin) -> Result<Self, PluginErrorCodes> {
        let mut plugin_metadata = Self {
            description: None,
            version: String::new(),
            name: String::new(),
            filename: plugin.filename.clone(),
            objfile: String::new(),
        };

        let contents = fs::read_to_string("metadata.toml")
            .map_err(|e| {
                log::error!("Couldn't read metadata file: {}", e.to_string());
                PluginErrorCodes::ParametersError
            });
        let buffer = String::from(contents.unwrap());

        let data_raw : Data = match toml::from_str(&buffer) {
            Ok(ok) => ok,
            Err(_) => return Err(PluginErrorCodes::ParametersError),
        };

        plugin_metadata.filename = "metadata.toml".to_owned();
        plugin_metadata.version = data_raw.metadata.version;
        plugin_metadata.name = data_raw.metadata.name;
        plugin_metadata.objfile = data_raw.metadata.objfile;

        Ok(plugin_metadata)
    }
}