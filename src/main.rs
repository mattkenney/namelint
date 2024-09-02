use std::io;
use std::fs::{self};
use std::path::Path;
use std::collections::VecDeque;
use std::ffi::OsString;
use clap::{arg, Arg, Command};
use regex::RegexBuilder;

//struct for configuration
struct Config {
    allow_msdos: bool,
}

fn main() {
    let matches = Command::new("namelint")
        .version("1.0")
        .about("lint your file names")
        .arg(Arg::new("msdos")
            .default_missing_value("true")
            .default_value("false")
            .help("Allow MS-DOS reserved file names (LPT1, COM1, etc.)")
            .long("allow-msdos")
            .num_args(0..=1)
            .require_equals(true) 
            .value_name("true|false")
            .value_parser(clap::builder::BoolishValueParser::new())
        )
        .arg(arg!(<path>... "paths to check")
            .help("Path(s) to checks")
            .trailing_var_arg(true))
        .get_matches();

    let config = Config {
        allow_msdos: *matches.get_one::<bool>("msdos").unwrap(),
    };

    let path_args: Vec<_> = matches.get_many::<String>("path")
        .expect("at least one path is required")
        .collect();

    let mut path_queue:VecDeque::<OsString> = VecDeque::new();
    for path_arg in path_args.iter() {
        let path_arg_str = *path_arg;
        path_queue.push_back(OsString::from(path_arg_str.clone()));
    }
    while path_queue.len() > 0 {
        let next_path = path_queue.pop_front().unwrap();
        let new_paths = visit_dirs(&config, &next_path).unwrap_or_else(|e| panic!("Unable to visit directory '{}': {}", next_path.to_string_lossy(), e));
        for new_path in new_paths.iter() {
            path_queue.push_back(new_path.clone());
        }
    }
}

/* check a file name for MS-DOS reserved names */
fn check_msdos(name: &str) -> bool {
    let msdos_reserved_pattern = RegexBuilder::new(r"(^|/)(CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])($|/|[.])")
        .case_insensitive(true)
        .build().unwrap();
    msdos_reserved_pattern.is_match(name)
}

/* check all files in a directory, and return a list (possibly empty) of subdirectories */
fn visit_dirs(config: &Config, dir: &OsString) -> io::Result< Vec<OsString> > {

    let dir_str = std::str::from_utf8(dir.as_encoded_bytes());
    if dir_str.is_err() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path is not UTF-8"));
    }
    let dir_str = dir_str.unwrap();
    if (config.allow_msdos == false) && check_msdos(dir_str) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "MS-DOS reserved name"));
    }

    let mut new_dirs:Vec::<OsString> = Vec::new();
    println!("Processing {:?}", dir);
    let path = Path::new(dir_str);
    if path.exists() == false {
        //println!("Path does not exist: {:?}", dir);
        return Err(io::Error::new(io::ErrorKind::NotFound, "Path does not exist"));
    }
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                new_dirs.push(entry_path.into_os_string());
            } else {
                println!("File: {:?}", &entry_path);
            }
        }
    }
    Ok(new_dirs)
}