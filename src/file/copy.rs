use wax::WalkEntry;
use std::fs;
use std::path::PathBuf;

use wax::{Glob, LinkBehavior};
use log::*;

pub fn dir(from: &PathBuf, to: &PathBuf, ignore_dir: String) -> Result<(), std::io::Error> {
    let ignore_list: Vec<&str> = ignore_dir
        .split("\n")
        .collect();
    let glob = Glob::new("**/*").unwrap();
    let depth = from.components().count();
    debug!("Walking {:?}, ignoring {:?}", from, ignore_list);
    for entry in glob
        .walk_with_behavior(from, LinkBehavior::ReadFile)
        .not(ignore_list)
        .unwrap()
    {
        let entry: WalkEntry = entry.unwrap();
        let src = entry.path();
        let mut iterator = src.components();
        iterator.advance_by(depth).unwrap();

        let mut dest = to.clone();
        while let Some(p) = iterator.next() {
            debug!("Adding {:?} to dest {:?}", p, dest);
            dest = dest.join(p);
        }

        let folder = dest.parent().unwrap();
        if fs::metadata(&folder).is_err() {
            debug!("mkdir: {:?}", folder);
            fs::create_dir_all(&folder)?;
        }
        if entry.file_type().is_file() {
            debug!("copy: {:?} -> {:?}", &src, &dest);
            fs::copy(&src, &dest)?;
        }
    }

    info!("Copied all files");

    Ok(())
}
