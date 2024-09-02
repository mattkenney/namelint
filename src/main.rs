use std::io;
use std::fs::{self};
use std::path::Path;
use std::collections::VecDeque;
use std::ffi::OsString;
use clap::{arg, Command};

fn main() {
    let matches = Command::new("namelint")
        .version("1.0")
        .about("lint your file names")
        .arg(arg!(<path>... "paths to check")
            .help("Path(s) to checks")
            .trailing_var_arg(true))
        .get_matches();

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
        let new_paths = visit_dirs(Path::new(&next_path)).unwrap_or_else(|e| panic!("Unable to visit directory '{}': {}", next_path.to_string_lossy(), e));
        for new_path in new_paths.iter() {
            path_queue.push_back(new_path.clone());
        }
    }
}

/* check all files in a directory, and return a list (possibly empty) of subdirectories */
fn visit_dirs(dir: &Path) -> io::Result< Vec<OsString> > {
    let mut new_dirs:Vec::<OsString> = Vec::new();
    println!("Processing {:?}", dir);
    if dir.exists() == false {
        //println!("Path does not exist: {:?}", dir);
        return Err(io::Error::new(io::ErrorKind::NotFound, "Path does not exist"));
    }
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                new_dirs.push(entry.path().into_os_string());
            } else {
                println!("File: {:?}", &path);
            }
        }
    }
    Ok(new_dirs)
}