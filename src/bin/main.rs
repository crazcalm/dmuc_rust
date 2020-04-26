//!dmuc 0.1
//!Marcus Willock <crazcalm@gmail.com>
//!A tool to help discover the terminal commands on your Unix Based Machine.
//!
//!USAGE:
//!    main [FLAGS] [OPTIONS]
//!
//!FLAGS:
//!    -a, --all        List files from both /usr/bin and /usr/local/bin directory
//!    -h, --help       Prints help information
//!    -l, --local      List files from /usr/local/bin directory
//!    -V, --version    Prints version information
//!
//!OPTIONS:
//!    -e, --endswith <String>      Filters output based on it the content ends with the provided string.
//!    -i, --inlcude <String>       Filters output based on if the content includes the provided string.
//!    -s, --startswith <String>    Filters output based on if the content starts with the provided string.

#[macro_use]
extern crate clap;
use clap::App;

use dmuc::dmuc as dmuc_;
use dmuc::dmuc_with_list;
use dmuc::Filter;

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
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // If there are not flags, do the default
    if matches.args.is_empty() {
        let path = usr_bin.as_ref();

        dmuc_(path, &Filter::None);
    } else if matches.is_present(local) && matches.is_present(startswith) {
        let path = local_bin.as_ref();
        let filter = Filter::Startswith(matches.value_of(startswith).unwrap());

        dmuc_(path, &filter);
    } else if matches.is_present(local) && matches.is_present(includes) {
        let path = local_bin.as_ref();
        let filter = Filter::Includes(matches.value_of(includes).unwrap());

        dmuc_(path, &filter);
    } else if matches.is_present(local) && matches.is_present(endswith) {
        let path = local_bin.as_ref();
        let filter = Filter::Endswith(matches.value_of(endswith).unwrap());

        dmuc_(path, &filter);
    } else if matches.is_present(local) {
        let path = local_bin.as_ref();
        dmuc_(path, &Filter::None);
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
    } else if matches.is_present(includes) {
        let path = usr_bin.as_ref();
        let filter = Filter::Includes(matches.value_of(includes).unwrap());
        dmuc_(path, &filter);
    } else if matches.is_present(startswith) {
        let path = usr_bin.as_ref();
        let filter = Filter::Startswith(matches.value_of(startswith).unwrap());

        dmuc_(path, &filter);
    } else if matches.is_present(endswith) {
        let path = usr_bin.as_ref();
        let filter = Filter::Endswith(matches.value_of(endswith).unwrap());

        dmuc_(path, &filter);
    }
}
