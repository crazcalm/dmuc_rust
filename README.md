# dmuc_rust
This is a port of another project of mine called [Discover My Unix Commands](https://github.com/crazcalm/DiscoverMyUnixCommands) and my Go port of it called [dmuc](https://github.com/crazcalm/dmuc)

## What does this Application do?
This application allows you to list out the applications in the /usr/bin and /usr/local/bin directories. You may also use the "starts with" or "includes" filters to filter your results.

## Purpose:
I am porting this tool to Rust as a means to become more familiar with Rust. Also, post creating the MVP, I think I can turn this into a tutorial on how to create Rust Terminal Application.

## Interface:
```
dmuc 0.1
Marcus Willock <crazcalm@gmail.com>
A tool to help discover the terminal commands on your Unix Based Machine.

USAGE:
    main [FLAGS] [OPTIONS]

FLAGS:
    -a, --all        List files from both /usr/bin and /usr/local/bin directory
    -h, --help       Prints help information
    -l, --local      List files from /usr/local/bin directory
    -V, --version    Prints version information

OPTIONS:
    -e, --endswith <String>      Filters output based on it the content ends with the provided string.
    -i, --inlcude <String>       Filters output based on if the content includes the provided string.
    -s, --startswith <String>    Filters output based on if the content starts with the provided string.
```