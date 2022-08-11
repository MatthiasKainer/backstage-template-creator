#![feature(iter_advance_by)]

#[macro_use]
extern crate clap;
extern crate atty;
extern crate walkdir;

mod file;
mod models;

use models::Replacement;
use std::env;
use std::path;
use std::path::*;
use std::io;
use std::io::prelude::*;
use std::fs;
use std::vec::*;

use log::*;

fn read_lines() -> Result<Vec<Replacement>, std::io::Error> {
    let mut replacements: Vec<Replacement> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => {
                let mut parts = l.split("=>");
                if parts.clone().count() != 2 {
                    error!("Invalid format in line {} - needs a variable and a replacement value, separated by =>", l);
                } else {
                    replacements.push(Replacement {
                        text: parts.next().unwrap().to_string(),
                        with: parts.next().unwrap().to_string(),
                    })
                }
            },
            Err(e) => return Err(e)
        }
        
    }
    Ok(replacements)
}

fn perform(input: &PathBuf, output: &PathBuf, replacements: &Vec<Replacement>) -> io::Result<()> {
    info!("Removing target destination {:?}", output);
    if std::path::Path::new(output).exists() {file::delete(output)?;}
    info!("Copy from {:?} to {:?}", input, output);
    let data: String = match fs::read_to_string(".btcignore") {
        Ok(file) => file,
        Err(_) => String::from(""),
    };
    info!("Data from btcignore file: {:?}", data);
    file::copy::dir(input, output, data)?;
    file::rename::dir_and_file_rename(output, replacements)?;
    info!("Replacing content in files");
    file::replace::content(output, replacements)?;
    Ok(())
}

fn main() -> io::Result<()> {
    env_logger::init();
    let matches = clap_app!(templatify =>
        (version: "1.0")
        (author: "Matthias Kainer")
        (about: "Creates backstage.io templates from projects. For logging, set the RUST_LOG variable to error, warn, info or debug")
        (@arg INPUT: -i --input +takes_value "Sets the base directory of the project to transform. Defaults to current directory.")
        (@arg OUTPUT: -o --output +takes_value +required "Sets the base directory for the created project. Everything at this location will be deleted first.")
    ).get_matches();

    if atty::is(atty::Stream::Stdin) {
        println!("Add a line for each replacement in the format ");
        println!("NAME=>${{values.VARIABLE}}");
        println!("and press ctrl+d once done");
    }

    let input: path::PathBuf = match matches.value_of("INPUT") {
        Some(value) => path::PathBuf::from(value),
        None => env::current_dir().unwrap(),
    };
    let output: PathBuf = match matches.value_of("OUTPUT") {
        Some(value) => path::PathBuf::from(value),
        None => std::process::exit(1),
    };

    match read_lines() {
        Ok(replacements) => {
            perform(&input, &output, &replacements)?;
            info!("Done")
        },
        Err(e) => error!("Error: {}", e)
    }

    Ok(())
}
