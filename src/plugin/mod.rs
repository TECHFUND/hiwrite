#![allow(improper_ctypes_definitions)]
#![allow(dead_code)]

pub mod error;
mod r#impl;
pub mod manager;
pub mod metadata;
pub mod plugin;

pub use manager::*;
pub use plugin::*;
