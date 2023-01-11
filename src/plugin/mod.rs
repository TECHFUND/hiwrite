#![allow(improper_ctypes_definitions)]
extern crate libloading;
extern crate toml;
extern crate zip;

pub mod error;
pub mod manager;
pub mod metadata;
pub mod plugin;
mod r#impl;

pub use manager::*;
pub use plugin::*;
