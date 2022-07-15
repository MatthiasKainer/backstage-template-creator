extern crate atty;
extern crate walkdir;

use std::fs;
use std::io;
use std::path::*;

use log::*;

use walkdir::WalkDir;

fn replace_content(file: &Path, replacements: &Vec<crate::models::Replacement>) -> io::Result<()> {
    let data = match fs::read_to_string(file) {
        Ok(data) => data,
        Err(_) => return Ok(()),
    };
    let data = replacements.iter().fold(data, |data, replacement| {
        data.replace(&replacement.text, &replacement.with)
    });
    fs::write(file, data).expect("Unable to write file");
    Ok(())
}

pub fn content(output: &PathBuf, replacements: &Vec<crate::models::Replacement>) -> io::Result<()> {
    let walker = WalkDir::new(output).min_depth(1).contents_first(true);
    let ignore_list = ["template.yml", "template.yaml"];

    for entry in walker {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            if !ignore_list.iter().any(|&e| e == entry_path.clone().file_name().and_then(|s| s.to_str()).unwrap()) {
                debug!("Replace content in file {:?}", entry_path);
                replace_content(entry_path, &replacements)?;
            }
        } else {
            continue;
        }
    }

    info!("renamed all directory and files");
    Ok(())
}
