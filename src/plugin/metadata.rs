use serde::Deserialize;
use crate::plugin::error::PluginErrorCodes;
use crate::plugin::Plugin;

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

        let f = match File::open("metadata.toml") {
            Ok(val) => val,
            Err(e) => match e.kind() {
                PermissionDenied => return Err(PluginErrorCodes::PermissionDenied),
                Unsupported => {
                    return Err(PluginErrorCodes::InternalError {
                        err: "Unsupported file".into(),
                    })
                }
                NotFound => return Err(PluginErrorCodes::NoSuchFile),
                Interrupted => return Err(PluginErrorCodes::InvalidPlugin),
                UnexpectedEof => return Err(PluginErrorCodes::InvalidPlugin),
                OutOfMemory => {
                    return Err(PluginErrorCodes::InternalError {
                        err: "Host is out of memory".into(),
                    })
                }
                Other => {
                    return Err(PluginErrorCodes::InternalError {
                        err: "Unknown error.".into(),
                    })
                }
                _ => panic!(),
            },
        };

        let contents = match std::io::read_to_string(f) {
            Ok(contents) => contents,
            Err(e) => {
                log::error!("Error reading metadata string: {}.", e.to_string());
                return Err(PluginErrorCodes::ParametersError);
            }
        };
        let buffer = String::from(contents.as_str());

        let data_raw: Data = match toml::from_str(&buffer) {
            Ok(ok) => ok,
            Err(_) => return Err(PluginErrorCodes::ParametersError),
        };

        if data_raw.metadata.name.is_empty() || data_raw.metadata.name.contains(' ') {
            panic!(
                "
                                Attempted to use a plugin that has an empty name in its metadata or contains an
                                invalid character in the field.
                                "
            )
        }

        if data_raw.metadata.version.is_empty() || data_raw.metadata.version.contains(' ') {
            log::error!(
                                "
                                Detected either empty or invalid version string in metadata.toml (Plugin
                                '{}'
                                ", data_raw.metadata.name
                        );
        }

        plugin_metadata.filename = "metadata.toml".to_owned();
        plugin_metadata.version = data_raw.metadata.version;
        plugin_metadata.name = data_raw.metadata.name;
        plugin_metadata.objfile = data_raw.metadata.objfile;

        Ok(plugin_metadata)
    }
}