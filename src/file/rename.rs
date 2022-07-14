extern crate atty;
extern crate walkdir;

use std::ffi;
use std::io;
use std::path;
use std::path::*;

use log::*;

use walkdir::WalkDir;

fn stem_ext(path: &path::Path) -> Option<(ffi::OsString, ffi::OsString)> {
    let stem: &ffi::OsStr = match path.file_stem() {
        Some(stem) => stem,
        None => return None,
    };

    let ext = match path.extension() {
        Some(ext) => ext,
        None => ffi::OsStr::new(""),
    };

    Some((stem.to_os_string(), ext.to_os_string()))
}

fn rename_file(path: &Path, replacements: &Vec<crate::models::Replacement>) -> io::Result<()> {
    let (stem, ext) = match stem_ext(path) {
        Some((stem, ext)) => (stem, ext),
        None => return Ok(()),
    };

    let name = stem.clone().into_string().unwrap();
    let name = replacements.iter().fold(name, |name, replacement| {
        name.replace(&replacement.text, &replacement.with)
    });
    let name = ffi::OsString::from(name);

    // with_file_name plus with_extension doesn't work for files with multiple
    //  dots, so this is a workaround. 
    let name = if !ext.clone().is_empty() {
        format!(
            "{}.{}",
            name.into_string().unwrap(),
            ext.clone().to_os_string().into_string().unwrap()
        )
    } else {
        name.into_string().unwrap()
    };
    if path == path.with_file_name(name.clone()).as_path() {
        return Ok(());
    }

    debug!(
        "Renaming {:?} to {:?}",
        path,
        path.with_file_name(name.clone()).as_path()
    );
    std::fs::rename(path, path.with_file_name(name.clone()).as_path())
}

fn rename(path: &Path, replacements: &Vec<crate::models::Replacement>) -> io::Result<()> {
    let name = match path.file_name().and_then(|s| s.to_str()) {
        Some(p) => p,
        None => return Err(io::Error::from(io::ErrorKind::InvalidData)),
    }
    .to_string();
    let name = replacements.iter().fold(name, |name, replacement| {
        name.replace(&replacement.text, &replacement.with)
    });
    let name = ffi::OsString::from(name);
    let name = name.as_os_str();

    if path == path.with_file_name(name).as_path() {
        return Ok(());
    }

    debug!(
        "Rename path {:?} with {:?}",
        path,
        path.with_file_name(name).as_path()
    );
    std::fs::rename(path, path.with_file_name(name).as_path())
}

pub fn dir_and_file_rename(
    output: &PathBuf,
    replacements: &Vec<crate::models::Replacement>,
) -> io::Result<()> {
    let walker = WalkDir::new(output).min_depth(1).contents_first(true);

    for entry in walker {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            debug!("Rename path {:?}", entry_path);
            rename(entry_path, &replacements)?;
        } else if entry_path.is_file() {
            debug!("Rename file {:?}", entry_path);
            rename_file(entry_path, &replacements)?;
        } else {
            continue;
        }
    }

    info!("renamed all directory and files");
    Ok(())
}
