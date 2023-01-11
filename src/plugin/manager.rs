extern crate libloading;
use crate::error::PluginErrorCodes;
use libloading::Symbol;
use std::{
    env,
    ffi::{c_int, c_void},
};
use std::os::raw::c_int;
use crate::plugin::error::PluginErrorCodes;

use super::plugin::Plugin;

#[repr(C)]
pub struct Manager {
    injected_plugin: Vec<Plugin>,
    entry_point: String,
    err_code: u32,
}

pub type HiWriteHook = unsafe extern "C" fn(*mut c_void) -> c_int;

impl Manager {
    pub fn new() -> Self {
        #[cfg(unix)]
        Self {
            injected_plugin: Vec::new(),
            entry_point: String::from("plugin_init"),
            err_code: 0,
        }
    }

    pub fn init_plugin(&mut self, filename: &str) -> Result<Plugin, PluginErrorCodes> {
        Plugin::load(filename)
    }

    pub fn register_plugin(&mut self, plugin: Plugin) -> Result<(), PluginErrorCodes> {
        self.injected_plugin.push(plugin);
        Ok(())
    }

    pub fn set_entry_point(&mut self, entry_point: &str) {
        let entry_point_with_null = &format!("{}\0", entry_point);
        self.entry_point = String::from(entry_point_with_null)
    }

    pub fn get_hook(&mut self, plugin: &Plugin, hook: &str) -> Result<HiWriteHook, PluginErrorCodes> {
        plugin.get_hook(hook)
    }

    pub fn get_custom_hook<P, T>(
        &mut self,
        plugin: &Plugin,
        hook: &str,
    ) -> Result<unsafe extern "C" fn(P) -> T, PluginErrorCodes> {
        plugin.get_custom_hook(hook)
    }

    pub fn begin_plugin(&mut self, plugin: &mut Plugin) -> Result<(), PluginErrorCodes> {
        let plugin_entry: Symbol<unsafe extern "C" fn() -> i32>;
        unsafe {
            plugin_entry = match plugin.raw.as_ref().unwrap().get(self.entry_point.as_bytes()) {
                Ok(fnc) => fnc,
                Err(e) => {
                    return Err(PluginErrorCodes::FailedToInitialize);
                }
            };

            let entry = plugin_entry();
            if entry != 0 {
                return Err(PluginErrorCodes::FailedToInitialize);
            }
        }
        plugin.started = true;
        Ok(())
    }

    pub extern "C" fn shutdown(mut self) {
        for plugin in self.injected_plugin.iter_mut() {
            plugin
                .terminate()
                .unwrap_or_else(|_| log::warn!("Error occured while unloading plugin."));
        }
    }
}

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        let plug_dir = env::temp_dir().join("hiwrite_plugin");
        for plug in &mut self.injected_plugin {
            plug.terminate().unwrap_or_else(|e| {
                log::warn!(
                    "Couldn't unload hiwrite_plugin: {} (err {}). No cleanup will be performed.",
                    e.to_string(),
                    e.raw_os_error().unwrap()
                )});
            drop(plug);
        }
        match std::fs::remove_dir_all(&plug_dir) {
            Ok(()) => log::trace!("Removed directory: {}", plug_dir.display()),
            Err(e) => {
                log::warn!(
                    "Couldn't remove directory: {} (err {}). No cleanup will be performed.",
                    e.to_string(),
                    e.raw_os_error().unwrap()
                );
            }
        }
    }
}
