#![allow(dead_code)]

extern crate libloading;
extern crate log;

use crate::error::PluginErrorCodes;
use crate::VHook;
use libloading::{Library, Symbol};
use serde::Deserialize;
use std::env::{self};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::ErrorKind::{self, *};
use zip::ZipArchive;
use crate::metadata::PluginMetadata;
use crate::plugin::error::PluginErrorCodes;
use crate::plugin::metadata::PluginMetadata;
use crate::plugin::HiWriteHook;

type LaterInitialized<T> = Option<T>;
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

#[derive(Deserialize)]
struct Data {
    metadata: Metadata,
}

#[derive(Deserialize)]
struct Metadata {
    description: Option<String>,
    version: String,
    name: String,
    objfile: String,
}

#[derive(Debug)]
#[repr(C)]
pub struct Plugin {
    pub metadata: LaterInitialized<PluginMetadata>,
    pub(crate) filename: String,
    pub(crate) is_valid: bool,
    pub(crate) started: bool,
    pub(crate) raw: LaterInitialized<Library>,
    pub(crate) archive: ZipArchive<File>,
}
