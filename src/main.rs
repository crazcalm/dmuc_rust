#[macro_use]
extern crate clap;
use clap::App;

use std::ffi::OsStr;
use std::path::PathBuf;
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

    // Sorts the entries by their path
    entries.sort();

    /*
    // The entries have now been sorted by their path.
    for file in &entries {
        if let Some(cmd) = file.file_name() {
            println!("file: {:?}", cmd);
        }
    }
    */

    Ok(entries)
}

fn print_to_screen(paths: Vec<PathBuf>, header: &str) {
    println!("{}:\n", header);

    for path in paths {
        match path.file_name() {
            Some(file_name) => match file_name.to_str() {
                Some(name) => {
                    println!("{}", name);
                }
                None => {
                    eprintln!("unable to convert file name into string: {:?}", file_name);
                }
            },
            None => {
                eprintln!("Unable to convert file path to string: {:?}", path);
            }
        }
    }
}

pub enum Filter<'a> {
    Startswith(&'a str),
    Endswith(&'a str),
    Includes(&'a str),
    None,
}

fn dmuc(path: &path::Path, filter: &Filter) {
    let mut results = ls_attempt(path).unwrap();

    let results = match filter {
        Filter::Startswith(string) => start_with_list(results, string),
        Filter::Includes(string) => includes_list(results, string),
        Filter::Endswith(string) => ends_with_list(results, string),
        Filter::None => results,
    };

    print_to_screen(results, path.to_str().unwrap());
}

fn dmuc_with_list(paths: Vec<&path::Path>, filter: &Filter) {
    for path in paths {
        dmuc(path, &filter);
    }
}

fn main() {
    //Defining my directory variables
    let usr_bin = "/usr/bin";
    let local_bin = "/usr/local/bin";
    let both_dirs = vec![usr_bin.as_ref(), local_bin.as_ref()];


    // CMD flags
    let local = "local";
    let all = "all";
    let startswith = "startswith";
    let includes = "includes";
    let endswith = "endswith";

    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // If there are not flags, do the default
    if matches.args.is_empty() {
        let path = usr_bin.as_ref();

        dmuc(path, &Filter::None);

    } else if matches.is_present(local) && matches.is_present(startswith) {
        let path = local_bin.as_ref();
        let filter = Filter::Startswith(matches.value_of(startswith).unwrap());

        dmuc(path, &filter);
    } else if matches.is_present(local) && matches.is_present(includes) {
        let path = local_bin.as_ref();
        let filter = Filter::Includes(matches.value_of(includes).unwrap());

        dmuc(path, &filter);
    } else if matches.is_present(local) && matches.is_present(endswith) {
        let path = local_bin.as_ref();
        let filter = Filter::Endswith(matches.value_of(endswith).unwrap());

        dmuc(path, &filter);

    } else if matches.is_present(local) {
        let path = local_bin.as_ref();
        dmuc(path, &Filter::None);

    } else if matches.is_present(all) && matches.is_present(startswith) {
        let filter = Filter::Startswith(matches.value_of(startswith).unwrap());
        dmuc_with_list(both_dirs, &filter);

    } else if matches.is_present(all) && matches.is_present(includes) {
        let filter = Filter::Includes(matches.value_of(includes).unwrap());
        dmuc_with_list(both_dirs, &filter);

    } else if matches.is_present(all) && matches.is_present(endswith) {
        let filter = Filter::Endswith(matches.value_of(endswith).unwrap());
        dmuc_with_list(both_dirs, &filter);

    } else if matches.is_present(all) {
        dmuc_with_list(both_dirs, &Filter::None)
    }else if matches.is_present(includes) {
        let path = usr_bin.as_ref();
        let filter = Filter::Includes(matches.value_of(includes).unwrap());
        dmuc(path, &filter);
    } else if matches.is_present(startswith) {
        let path = usr_bin.as_ref();
        let filter = Filter::Startswith(matches.value_of(startswith).unwrap());

        dmuc(path, &filter);
    } else if matches.is_present(endswith) {
        let path = usr_bin.as_ref();
        let filter = Filter::Endswith(matches.value_of(endswith).unwrap());

        dmuc(path, &filter);
    }
}
