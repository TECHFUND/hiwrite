#![allow(dead_code)]

use crate::plugin::metadata::PluginMetadata;
use libloading::Library;
use serde::Deserialize;
use std::fs::File;
use zip::ZipArchive;

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
