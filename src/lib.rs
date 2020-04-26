//!# dmuc_rust
//!This is a port of another project of mine called [Discover My Unix Commands](https://github.com/crazcalm/DiscoverMyUnixCommands) and my Go port of it called [dmuc](https://github.com/crazcalm/dmuc)
//!
//!## What does this Application do?
//!This application allows you to list out the applications in the /usr/bin and /usr/local/bin directories. You may also use the "starts with" or "includes" filters to filter your results.
//!
//!## Purpose:
//!I am porting this tool to Rust as a means to become more familiar with Rust. Also, post creating the MVP, I think I can turn this into a tutorial on how to create Rust Terminal Application.
//!
//!
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

///Filter enums are used to denote which type of filtering will be used in dmuc
pub enum Filter<'a> {
    Startswith(&'a str),
    Endswith(&'a str),
    Includes(&'a str),
    None,
}

///dmuc holds the core logic of the application.
/// # Example
/// ```
/// use dmuc::dmuc as dmuc_;
/// use dmuc::Filter;
///
/// let path = "/usr/bin".as_ref();
///
/// dmuc_(path, &Filter::None);
/// ```
pub fn dmuc(path: &path::Path, filter: &Filter) {
    let mut results = ls_attempt(path).unwrap();

    let results = match filter {
        Filter::Startswith(string) => start_with_list(results, string),
        Filter::Includes(string) => includes_list(results, string),
        Filter::Endswith(string) => ends_with_list(results, string),
        Filter::None => results,
    };

    print_to_screen(results, path.to_str().unwrap());
}

///dmuc_with list is small wrapper of dmuc
/// # Example
/// ```
/// use dmuc::Filter;
/// use dmuc::dmuc_with_list;
///
/// let paths = vec!["/usr/bin".as_ref(), "/usr/local/bin".as_ref()];
///
/// dmuc_with_list(paths, &Filter::None);
/// ```
pub fn dmuc_with_list(paths: Vec<&path::Path>, filter: &Filter) {
    for path in paths {
        dmuc(path, &filter);
    }
}

#[cfg(test)]
mod tests {
    use self::super::*;
    use std::collections::HashMap;

    #[test]
    fn test_start_with_check(){
        let mut test_cases = vec![
            ("name", "na", true),
            ("name", "me", false),
            ("name", "no", false),
            ("name", "", true),
        ];

        for case in test_cases {
            let result = start_with_check(case.0, case.1);

            assert_eq!(result, case.2);
        }
    }

    #[test]
    fn test_includes_check(){
        let mut test_cases = vec![
            ("name", "na", true),
            ("name", "me", true),
            ("name", "no", false),
            ("name", "", true),
        ];

        for case in test_cases {
            let result = includes_check(case.0, case.1);

            assert_eq!(result, case.2);
        }
    }

    #[test]
    fn test_ends_with_check(){
        let mut test_cases = vec![
            ("name", "na", false),
            ("name", "me", true),
            ("name", "no", false),
            ("name", "", true),
        ];

        for case in test_cases {
            let result = ends_with_check(case.0, case.1);

            assert_eq!(result, case.2);
        }
    }

}