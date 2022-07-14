use std::fs;
use std::path::{PathBuf};

use log::*;

pub fn dir(from: &PathBuf, to: &PathBuf) -> Result<(), std::io::Error> {
    let ignore_list = ["node_modules", ".git", "dist", ".parcel-cache"];
    let mut stack: Vec<PathBuf> = Vec::new();
    stack.push(from.clone());

    let output_root = to;
    let input_root = from.components().count();

    while let Some(working_path) = stack.pop() {
        debug!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            debug!(" mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if !ignore_list.iter().any(|&e| e == path.clone().file_name().and_then(|s| s.to_str()).unwrap()) {
                    stack.push(path);
                }
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        debug!("  copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        debug!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    info!("Copied all files");

    Ok(())
}