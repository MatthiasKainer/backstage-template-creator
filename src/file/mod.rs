use std::fs::remove_dir_all;
use std::path::PathBuf;

pub mod copy;
pub mod rename;
pub mod replace;

pub fn delete(path: &PathBuf) -> std::io::Result<()> {
    return remove_dir_all(path);
}