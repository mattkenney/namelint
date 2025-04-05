mod safe_name;
mod load;
mod parse_rules;
mod schema;
mod structs;
mod rules;

use std::fs::{self};
use std::path::{Path, PathBuf};
use std::collections::{HashMap, VecDeque};
use std::ffi::OsString;
use clap::{arg, Command};
use include_dir::{include_dir, Dir};
use safe_name::{safe_os_string, safe_string};

use parse_rules::parse_rules;
use schema::must_load_validator;
use structs::{Rule, RuleSet};

static RULES_DIR: Dir = include_dir!("./rules");

fn main() {

	let mut command = Command::new("namelint")
		.version("1.0")
		.about("Check file names for security, compatibility, best practices & standards.")
		;

	command = command.arg(arg!(--config <FILE> "Specify an alternate .namelint file")
		.required(false)
		.value_parser(clap::value_parser!(String)));

	command = command.arg(arg!(--rules <FILE> "Specify additional rule definitions to load")
		.required(false)
		.action(clap::ArgAction::Append)
		.value_parser(clap::value_parser!(String)));

	command = command.arg(arg!(-v --verbose "More verbose output: repeat for even more")
		.required(false)
		.action(clap::ArgAction::Count));

	command = command.arg(
		arg!(--schema <FILE> "Specify an alternate schema file")
			.required(false)
			.value_parser(clap::value_parser!(String)),
	);

	command = command.arg(arg!(<path>... "paths to check")
			.help("Path(s) to checks")
			.trailing_var_arg(true));

	let binding = command.get_matches();
	//LATER: convert to log level let verbose = binding.get_count("verbose");
	let schema_file = binding.get_one::<String>("schema");
	let validator = must_load_validator(schema_file);

	let mut all_rules: HashMap<String, Rule> = HashMap::new();
	let mut all_rulesets: HashMap<String, RuleSet> = HashMap::new();

	for rule_file in RULES_DIR.find("*.yaml").unwrap() {
		let body = RULES_DIR.get_file(rule_file.path()).unwrap().contents_utf8().unwrap();
		let src = rule_file.path().display().to_string();
		parse_rules(body, &src, &validator, &mut all_rules, &mut all_rulesets);
	}
	println!("INFO: loaded {} rules and {} rulesets from built-in files", all_rules.len(), all_rulesets.len());

	if binding.contains_id("rules") {
		println!("DEBUG: there are custom rules");
		let mut custom_rules = binding.get_many::<String>("rules")
			.unwrap()
			.map(|s| s.as_str());

		while let Some(custom_rule) = custom_rules.next() {
			let body = fs::read_to_string(custom_rule)
				.unwrap_or_else(|e| panic!("Unable to read custom rule file {}: {}", custom_rule, e));
			parse_rules(&body, custom_rule, &validator, &mut all_rules, &mut all_rulesets);
		}
	}
/*
	let mut selected_rules: HashMap<String, &RuleRegex> = HashMap::new();
	for (rule_id, rule_regex) in all_rules.iter() {
		if *matches.get_one::<bool>(rule_id).unwrap() {
			println!("Rule {} enabled ({})", rule_id, rule_regex.pattern);
			selected_rules.insert(rule_id.to_string(), rule_regex);
		}
	}
*/
	let path_args: Vec<_> = binding.get_many::<String>("path")
		.expect("at least one path is required")
		.collect();

	let selected_rules: Vec<Rule> = Vec::new();

	let mut path_queue:VecDeque::<PathBuf> = VecDeque::new();
	for path_arg in path_args.iter() {
		let path_arg_str = *path_arg;
		let path = Path::new(path_arg_str);
		if path.exists() == false {
			println!("ERROR: Path does not exist: '{}'", path_arg_str);
			continue;
		}
		if path.is_dir() {
			println!("DEBUG: queueing directory '{}'", path_arg_str);
			//LATER: also print canonical path
			path_queue.push_back(path.to_path_buf());
		} else if path.is_file() {
			//LATER: should this be a warning, since they're probably using shell wildcards?
			println!("DEBUG: processing file from command '{}'", path_arg_str);
			process_file(&selected_rules, path);
		} else {
			println!("ERROR: Path is not a file or directory: '{}'", path_arg_str);
		}
	}

	while path_queue.len() > 0 {
		let next_path = path_queue.pop_front();
		if next_path.is_none() {
			break;
		}
		let next_path = next_path.unwrap();

		let new_paths = visit_dir(&selected_rules, &next_path);
		if new_paths.is_err() {
			println!("ERROR: Unable to visit directory {}: {}", next_path.to_string_lossy(), new_paths.err().unwrap());
			continue;
		}
		let new_paths = new_paths.unwrap();
		for new_path in new_paths.iter() {
			path_queue.push_back(new_path.clone());
		}
	}
}

fn test_name(_rules: &Vec<Rule>, name: &OsString) -> core::result::Result<(), String>{
	let name_str = std::str::from_utf8(name.as_encoded_bytes());
	if name_str.is_err() {
		return Err("ERROR: Path is not UTF-8".to_string());
	}
	/*
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
	*/
	return Ok(());
}

fn process_file(rules: &Vec<Rule>, path: &Path) {
	let file_name = path.file_name();
	if file_name.is_none() {
		println!("ERROR: Unable to get file name for {:?}", path);
		return;
	}
	let file_name = file_name.unwrap();
	let results = test_name(rules, &file_name.to_os_string());
	if results.is_err() {
		println!("ERROR: Invalid file {}: {}", path.to_string_lossy(), results.unwrap_err());
	}
/*
			if rules::nfc::nfc(entry_path.to_string_lossy()) == false {
				println!("WARNING: Non-NFC filename: {:?}", &entry_path);
			}
*/
}

/* check all files in a directory, and return a list (possibly empty) of subdirectories */
fn visit_dir(rules: &Vec<Rule>, dir: &PathBuf) -> Result< Vec<PathBuf>, String > {

	let mut new_dirs:Vec::<PathBuf> = Vec::new();

	let dir_str = dir.to_str();
	if dir_str.is_none() {
		return Err("Directory name is not valid unicode".to_string());
	}
	let dir_str = dir_str.unwrap();

	println!("INFO: Processing directory '{:?}'", safe_string(dir_str));
	let path = dir.as_path();
	let canonical_path = path.canonicalize();
	if canonical_path.is_err() {
		return Err(format!("Unable to canonicalize directory {}: {}", safe_string(dir_str), canonical_path.err().unwrap()));
	}
	let canonical_path = canonical_path.unwrap();
	if canonical_path != path {
		println!("WARNING: canonicalized path is different: '{}'", safe_os_string(&canonical_path.as_os_str().to_os_string()));
	}

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

	let dir_entry = fs::read_dir(path);
	if dir_entry.is_err() {
		return Err(format!("Unable to read directory {}: {}", path.to_string_lossy(), dir_entry.err().unwrap()));
	}
	let dir_entry = dir_entry.unwrap();

	for entry in dir_entry {
		if entry.is_err() {
			println!("ERROR: Unable to read directory entry: {}", entry.err().unwrap());
			continue;
		}
		let entry = entry.unwrap();
		let entry_path = entry.path();
		if entry_path.is_dir() {
			new_dirs.push(entry_path);
		} else {
			process_file(rules, &entry_path);
		}
	}
	Ok(new_dirs)
}



