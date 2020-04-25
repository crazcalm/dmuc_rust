#[macro_use]
extern crate clap;
use clap::App;

use std::{fs, io, path};

fn start_with_check(item: &str, start_with: &str) -> bool {
    item.starts_with(start_with)
}

fn ends_with_check(item: &str, ends_with: &str) -> bool {
    item.ends_with(ends_with)
}

fn includes_check(item: &str, includes: &str) -> bool {
    item.contains(includes)
}

fn ends_with_list(mut list: Vec<path::PathBuf>, ends_with: &str) -> Vec<path::PathBuf> {
    let mut results: Vec<path::PathBuf> = Vec::new();

    for item in list {
        if let Some(file_name) = item.file_name() {
            if let Some(file_name_string) = file_name.to_str() {
                if ends_with_check(file_name_string, ends_with) {
                    results.push(item);
                }
            }
        }
    }

    results
}

fn includes_list(mut list: Vec<path::PathBuf>, includes: &str) -> Vec<path::PathBuf> {
    let mut results: Vec<path::PathBuf> = Vec::new();

    for item in list {
        if let Some(file_name) = item.file_name() {
            if let Some(file_name_string) = file_name.to_str() {
                if includes_check(file_name_string, includes) {
                    results.push(item);
                }
            }
        }
    }

    results
}

fn start_with_list(mut list: Vec<path::PathBuf>, start_with: &str) -> Vec<path::PathBuf> {
    let mut results: Vec<path::PathBuf> = Vec::new();

    for item in list {
        if let Some(file_name) = item.file_name() {
            if let Some(file_name_string) = file_name.to_str() {
                if start_with_check(file_name_string, start_with) {
                    results.push(item);
                }
            }
        }
    }

    results
}

fn ls_attempt(path: &path::Path) -> io::Result<Vec<path::PathBuf>> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    // The entries have now been sorted by their path.

    for file in &entries {
        if let Some(cmd) = file.file_name() {
            println!("file: {:?}", cmd);
        }
    }

    Ok(entries)
}

fn main() {
    println!("Hello, world!");

    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    println!("matches: {:?}", matches);

    if let Some(v) = matches.value_of("includes") {
        println!("includes: {:?}", v);
    }

    let all = matches.is_present("all");

    if all {
        println!("all is present");
    } else {
        println!("all is not present");
    }

    let usr_bin = "/usr/bin";
    let local_bin = "/usr/local/bin";

    ls_attempt(local_bin.as_ref());

    let mut testing = ls_attempt(usr_bin.as_ref()).unwrap();

    println!("pre-testing: {:?}", testing);

    let mut testing2 = start_with_list(testing.clone(), "zip");

    println!("starts_with: {:?}", testing2);

    let mut testing2 = ends_with_list(testing.clone(), "zip");

    println!("ends_with: {:?}", testing2);

    let mut testing2 = includes_list(testing.clone(), "zip");

    println!("includes: {:?}", testing2);
}
