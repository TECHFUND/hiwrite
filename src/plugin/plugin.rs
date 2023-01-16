#![allow(dead_code)]

extern crate libloading;
extern crate log;

use libloading::{Library, Symbol};
use serde::Deserialize;
use std::env::{self};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::ErrorKind::{self, *};
use zip::ZipArchive;
use crate::plugin::error::PluginErrorCodes;
use crate::plugin::metadata::PluginMetadata;
use crate::plugin::HiWriteHook;

type LaterInitialized<T> = Option<T>;

#[derive(Deserialize)]
pub struct Data {
    pub(crate) metadata: Metadata,
}

#[derive(Deserialize)]
pub struct Metadata {
    description: Option<String>,
    pub(crate) version: String,
    pub(crate) name: String,
    pub(crate) objfile: String,
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
