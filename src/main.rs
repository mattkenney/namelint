use std::io;
use std::fs::{self};
use std::path::Path;
use std::collections::VecDeque;
use std::ffi::OsString;


/* check all files in a directory, and return a list (possibly empty) of subdirectories */
fn visit_dirs(dir: &Path) -> io::Result< Vec<OsString> > {
    let mut new_dirs:Vec::<OsString> = Vec::new();
    println!("Processing {:?}", dir);
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

fn main() {
    let mut path_queue:VecDeque::<OsString> = VecDeque::new();
    path_queue.push_back(OsString::from("."));
    while path_queue.len() > 0 {
        let next_path = path_queue.pop_front().unwrap();
        let new_paths = visit_dirs(Path::new(&next_path));
        if new_paths.is_err() { continue; }
        if let Ok(new_paths) = new_paths {
            for new_path in new_paths.iter() {
            path_queue.push_back(new_path.clone());
            }
        }
    }
}