use std::ffi::OsStr;
use std::{env, fs};
use std::io::ErrorKind;
use std::io::ErrorKind::{Interrupted, NotFound, Other, OutOfMemory, PermissionDenied, UnexpectedEof, Unsupported};
use crate::plugin::error::PluginErrorCodes;
use crate::plugin::{HiWriteHook, Plugin};
use crate::plugin::metadata::PluginMetadata;
use libloading::{Library, Symbol};

macro_rules! initialize_later {
    () => {
        None
    };
}

macro_rules! init_now {
    ($a:expr) => {
        Some($a)
    };
}


impl Plugin {
    fn load_archive<S: Copy + Into<String> + AsRef<OsStr>>(
        filename: S,
    ) -> Result<Self, PluginErrorCodes> {
        let tmp = filename.into();
        let fname = std::path::Path::new(&tmp);
        let file = match fs::File::open(fname) {
            Ok(val) => val,
            Err(e) => {
                match e.kind() {
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
                }
            }
        };

        match std::fs::create_dir(env::temp_dir().join("hiwrite_plugin")) {
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => (),
                _ => log::info!("Couldn't create directory: {}", e.to_string()),
            },
            Ok(_) => env::set_current_dir(env::temp_dir().join("hiwrite_plugin")).unwrap(),
        }

        let mut archive = zip::ZipArchive::new(file).unwrap();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            if (*file.name()).ends_with('/') {
                fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).unwrap();
                    }
                }

                let mut outfile = fs::File::create(&outpath).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            }
        }

        let plugin = Self {
            metadata: initialize_later!(),
            raw: initialize_later!(),
            filename: filename.into(),
            is_valid: false,
            started: false,
            archive,
        };

        Ok(plugin)
    }
    pub fn load<S: Copy + Into<String> + AsRef<OsStr>>(
        filename: S,
    ) -> Result<Plugin, PluginErrorCodes> {
        let mut plugin = match Self::load_archive(filename) {
            Err(e) => {
                log::error!("Couldn't load archive, stopping here.");
                return Err(e);
            }
            Ok(p) => p,
        };

        #[allow(deprecated)]
        match plugin.load_metadata() {
            Err(e) => {
                return Err(e);
            }
            Ok(_) => {
                fs::create_dir_all(
                    env::temp_dir()
                        .join("hiwrite_plugin")
                        .join(&plugin.metadata.as_ref().unwrap().name),
                )
                .expect("Cannot create plugin directory!");
            }
        }
        Ok(plugin)
    }

    pub(in crate::plugin) fn load_vhook(&self, fn_name: &str) -> Result<HiWriteHook, PluginErrorCodes> {
        if !self.started || !self.is_valid || self.raw.is_none() {
            return Err(PluginErrorCodes::InvalidPlugin);
        }
        let hook: Symbol<HiWriteHook>;
        unsafe {
            hook = match self
                .raw
                .as_ref()
                .unwrap_unchecked()
                .get(format!("{}\0", fn_name).as_bytes())
            {
                Ok(v) => v,
                Err(_) => return Err(PluginErrorCodes::MissingSymbol),
            };
        }
        Ok(*hook)
    }

    pub(crate) fn get_hook(&self, fn_name: &str) -> Result<HiWriteHook, PluginErrorCodes> {
        Self::load_vhook(self, fn_name)
    }

    pub(crate) fn get_custom_hook<P, T>(
        &self,
        fn_name: &str,
    ) -> Result<unsafe extern "C" fn(P) -> T, PluginErrorCodes> {
        if !self.started || !self.is_valid || self.raw.is_none() {
            return Err(PluginErrorCodes::InvalidPlugin);
        }
        let hook: Symbol<unsafe extern "C" fn(P) -> T>;
        unsafe {
            hook = match self
                .raw
                .as_ref()
                .unwrap_unchecked()
                .get(format!("{}\0", fn_name).as_bytes())
            {
                Ok(v) => v,
                Err(_) => return Err(PluginErrorCodes::MissingSymbol),
            };
        }
        Ok(*hook)
    }

    pub fn get_metadata(&self) -> &Option<PluginMetadata> {
        &self.metadata
    }

    pub fn load_metadata(&mut self) -> Result<(), PluginErrorCodes> {
        match PluginMetadata::load(self) {
            Ok(v) => {
                let plugin_dir_name = env::temp_dir().join("hiwrite_plugin").join(&v.name);

                fs::create_dir_all(&plugin_dir_name).unwrap();
                fs::copy(&v.objfile, plugin_dir_name.join(&v.objfile)).unwrap();

                self.raw =
                    unsafe { init_now!(Library::new(plugin_dir_name.join(&v.objfile)).unwrap()) };
                self.is_valid = true;
                self.metadata = init_now!(v);

                Ok(())
            }
            Err(e) => {
                log::error!(
                    "Couldn't load metadata ({}): {}",
                    self.filename,
                    e.to_string()
                );
                Err(e)
            }
        }
    }

    pub fn terminate(&mut self) -> Result<(), PluginErrorCodes> {
        if self.raw.is_none() {
            return Err(PluginErrorCodes::InvalidPlugin);
        }

        if !self.started {
            return Err(PluginErrorCodes::InvalidPlugin);
        }

        let destructor: Symbol<unsafe extern "C" fn() -> ()>;
        unsafe {
            destructor = match self.raw.as_ref().unwrap_unchecked().get(b"hiwrite_plugin_exit\0") {
                Ok(v) => v,
                Err(_) => {
                    return Err(PluginErrorCodes::InvalidPlugin);
                }
            };

            destructor();
        }

        self.started = false;
        if cfg!(feature = "non_reusable_plugins") {
            self.is_valid = false;
            self.raw = None;
            self.filename = String::new();
            self.metadata = None;
        }
        Ok(())
    }

    pub fn is_function_available(&self, name: &str) -> bool {
        if self.raw.is_none() {
            return false;
        }
        unsafe {
            self.raw
                .as_ref()
                .unwrap()
                .get::<unsafe extern "C" fn()>(name.as_bytes())
                .is_ok()
        }
    }
    #[inline(always)]
    pub fn is_metadata_loaded(&self) -> bool {
        self.metadata.is_some()
    }
}

impl Drop for Plugin {
    fn drop(&mut self) {
        let plugin_dir_name = env::temp_dir()
            .join("hiwrite_plugin")
            .join(&self.metadata.as_ref().unwrap().name);

        match std::fs::remove_dir_all(&plugin_dir_name) {
            Err(e) => {
                log::error!("Couldn't remove plugin directory: {}", e.to_string());
            }
            Ok(_) => (),
        }
    }
}
