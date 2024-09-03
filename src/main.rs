use std::fs::{self};
use std::path::Path;
use std::collections::{HashMap, VecDeque};
use std::ffi::OsString;
use clap::{arg, Arg, Command};
use regex::{Regex, RegexBuilder};
use include_dir::{include_dir, Dir};
use serde_yaml;
use serde;

static RULES_DIR: Dir = include_dir!("./rules");

#[derive(serde::Deserialize)]
struct RuleRegex {
    pattern: String,
    #[serde(default)]
    case_insensitive: bool,
    #[serde(skip)]
    regex: Option<Regex>,
    result: String,
}

#[derive(serde::Deserialize)]
struct Rule {
    default_missing_value: String,
    default_value: String,
    description: String,
    help: String,
    long: String,
    regex: RuleRegex,
}

fn main() {

    // map of string to RuleRegex
    let mut all_rules: HashMap<String, RuleRegex> = HashMap::new();

    let mut command = Command::new("namelint")
        .version("1.0")
        .about("lint your file names")
        ;

    for rule_file in RULES_DIR.find("*.yaml").unwrap() {
        let body = RULES_DIR.get_file(rule_file.path()).unwrap().contents_utf8().unwrap();

        let id = rule_file.path().file_stem().unwrap().to_str().unwrap();
        println!("Loading rule {} ({})", id, rule_file.path().display());
        let mut rule: Rule = serde_yaml::from_str(body)
            .unwrap_or_else(|e| panic!("Unable to load rule {} ({}): {}", id, rule_file.path().display(), e));

        //create a new OsString from rule.default_missing_value
        let default_missing_value = clap::builder::OsStr::from(rule.default_missing_value);
        command = command.arg(Arg::new(id)
            .default_missing_value(default_missing_value)
            .default_value(clap::builder::OsStr::from(rule.default_value))
            .help(clap::builder::StyledStr::from(rule.help))
            .long(clap::builder::Str::from(rule.long))
            .num_args(0..=1)
            .require_equals(true) 
            .value_name("true|false")
            .value_parser(clap::builder::BoolishValueParser::new())
        );
        let the_regex = Some(RegexBuilder::new(&rule.regex.pattern)
            .case_insensitive(rule.regex.case_insensitive)
            .build()
            .unwrap_or_else(|e| panic!("Unable to compile regex for rule {}: {}", id, e)));
        rule.regex.regex = the_regex;
        all_rules.insert(id.to_string(), rule.regex);
    }
    command = command.arg(arg!(<path>... "paths to check")
            .help("Path(s) to checks")
            .trailing_var_arg(true));

    let matches = command.get_matches();

    let mut selected_rules: HashMap<String, &RuleRegex> = HashMap::new();
    for (rule_id, rule_regex) in all_rules.iter() {
        if *matches.get_one::<bool>(rule_id).unwrap() {
            println!("Rule {} enabled ({})", rule_id, rule_regex.pattern);
            selected_rules.insert(rule_id.to_string(), rule_regex);
        }
    }

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
        let new_paths = visit_dir(&selected_rules, &next_path).unwrap_or_else(|e| panic!("Unable to visit directory '{}': {}", next_path.to_string_lossy(), e));
        for new_path in new_paths.iter() {
            path_queue.push_back(new_path.clone());
        }
    }
}

fn test_name(rules: &HashMap<String, &RuleRegex>, name: &OsString) -> core::result::Result<(), String>{
    let name_str = std::str::from_utf8(name.as_encoded_bytes());
    if name_str.is_err() {
        return Err("Path is not UTF-8".to_string());
    }
    let name_str = name_str.unwrap();
    for (rule_id, rule_regex) in rules.iter() {
        let regex = rule_regex.regex.as_ref().unwrap();
        let found = regex.is_match(name_str);
        println!("Checking rule {} on {}: {}", rule_id, name_str, found);
        if found && rule_regex.result == "fail" {
            return Err(format!("Matched rule {}", rule_id));
        } else if found == false && rule_regex.result == "pass" {
            return Err(format!("Did not match rule {}", rule_id));
        }
    }
    return Ok(());
}


/* check all files in a directory, and return a list (possibly empty) of subdirectories */
fn visit_dir(rules: &HashMap<String, &RuleRegex>, dir: &OsString) -> Result< Vec<OsString>, String > {


    let mut new_dirs:Vec::<OsString> = Vec::new();
    println!("Processing {:?}", dir);
    let path = Path::new(dir);
    if path.file_name().is_some() {     // needed for root and . directory
        let dir_name = path.file_name().unwrap();
        let results = test_name(rules, &dir_name.to_os_string());
        if results.is_err() {
            return Err(format!("Invalid directory {}: {}", path.to_string_lossy(), results.unwrap_err()));
        }
    }
    if path.exists() == false {
        return Err(format!("Invalid directory {}: does not exist", path.to_string_lossy()));
    }

    if path.is_dir() == false {
        return Err(format!("Invalid directory {}: not a directory", path.to_string_lossy()));
    }

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_dir() {
            new_dirs.push(entry_path.into_os_string());
        } else {
            println!("File: {:?}", &entry_path);
        }
    }
    Ok(new_dirs)
}