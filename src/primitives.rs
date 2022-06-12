use std::{error::Error, path::PathBuf};

#[derive(Debug)]
pub struct Settings {
    pub git_dir: PathBuf,
    pub remote_alias: String,
    pub root: Key,
}

#[derive(Debug)]
pub struct Key {
    pub ips_id: u64,
    pub subasset_id: Option<u64>,
}

pub type BoxResult<T> = Result<T, Box<dyn Error>>;
