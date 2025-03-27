mod schema;
mod load;

use std::fs::{self};
use std::path::Path;
use std::collections::{HashMap, VecDeque};
use std::ffi::OsString;
use clap::{arg, Command};
use regex::{Regex, RegexBuilder};
use include_dir::{include_dir, Dir};
use serde_yaml;
use serde;

use schema::must_load_validator;
use load::{must_load_yaml, must_yaml_to_json};

static RULES_DIR: Dir = include_dir!("./rules");

#[derive(serde::Deserialize,Debug, Clone)]
struct RuleRegex {
	pattern: String,
	#[serde(default)]
	case_insensitive: bool,
	#[serde(skip)]
	regex: Option<Regex>,
	//result: String,
}

#[derive(serde::Deserialize,Debug, Clone)]
struct Rule {
	//description: String,
	rule_id: String,
	regex: Option<RuleRegex>,
	title: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
struct RuleSet {
	ruleset_id: String,
	//rules: Vec<String>,
	//rulesets: Vec<String>,
	title: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
struct RuleFile {
	rules: Option<Vec<Rule>>,
	rulesets: Option<Vec<RuleSet>>,
}

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

	command = command.arg(
		arg!(--schema <FILE> "Specify an alternate schema file")
			.required(false)
			.value_parser(clap::value_parser!(String)),
	);

	command = command.arg(arg!(<path>... "paths to check")
			.help("Path(s) to checks")
			.trailing_var_arg(true));

	let binding = command.get_matches();
	let schema_file = binding.get_one::<String>("schema");
	let validator = must_load_validator(schema_file);

	let mut all_rules: HashMap<String, Rule> = HashMap::new();
	let mut all_rulesets: HashMap<String, RuleSet> = HashMap::new();

	for rule_file in RULES_DIR.find("*.yaml").unwrap() {
		let body = RULES_DIR.get_file(rule_file.path()).unwrap().contents_utf8().unwrap();
		let src = rule_file.path().display().to_string();
		load_rules_from_str(body, &src, &validator, &mut all_rules, &mut all_rulesets);
	}
	println!("INFO: loaded {} rules and {} rulesets from built-in files", all_rules.len(), all_rulesets.len());

	if binding.contains_id("rules") {
		println!("DEBUG: there are custom rules");
	} else {
		println!("DEBUG: there are no custom rules");
	}
	let mut custom_rules = binding.get_many::<String>("rules")
		.unwrap()
		.map(|s| s.as_str());

	while let Some(custom_rule) = custom_rules.next() {
		let body = fs::read_to_string(custom_rule)
			.unwrap_or_else(|e| panic!("Unable to read custom rule file {}: {}", custom_rule, e));
		load_rules_from_str(&body, custom_rule, &validator, &mut all_rules, &mut all_rulesets);
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

	let mut path_queue:VecDeque::<OsString> = VecDeque::new();
	for path_arg in path_args.iter() {
		let path_arg_str = *path_arg;
		path_queue.push_back(OsString::from(path_arg_str.clone()));
	}

	let selected_rules: Vec<Rule> = Vec::new();

	while path_queue.len() > 0 {
		let next_path = path_queue.pop_front().unwrap();
		let new_paths = visit_dir(&selected_rules, &next_path).unwrap_or_else(|e| panic!("Unable to visit directory '{}': {}", next_path.to_string_lossy(), e));
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


/* check all files in a directory, and return a list (possibly empty) of subdirectories */
fn visit_dir(rules: &Vec<Rule>, dir: &OsString) -> Result< Vec<OsString>, String > {

	let mut new_dirs:Vec::<OsString> = Vec::new();
	println!("INFO: Processing {:?}", dir);
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
			println!("DEBUG: Processing file: {:?}", &entry_path);
		}
	}
	Ok(new_dirs)
}


fn load_rules_from_str(body: &str, src: &str, validator: &jsonschema::Validator, all_rules: &mut HashMap<String, Rule>, all_rulesets: &mut HashMap<String, RuleSet>) {
	let yaml_data = must_load_yaml(body, &src);
	let json_data = must_yaml_to_json(&yaml_data, &src);
	let is_valid = validator.validate(&json_data);
	if is_valid.is_err() {
		println!("ERROR: Invalid rules file {}: {}", src, is_valid.err().unwrap());
		// LATER: should this be a fatal error?
		return;
	}

	println!("DEBUG: Loading rules file {}", &src);
	let rulef: RuleFile = serde_yaml::from_str(body)
		.unwrap_or_else(|e| panic!("Unable to load rules file {}: {}", &src, e));

	if rulef.rulesets.is_some() {
		let mut rulesets = rulef.rulesets.unwrap();
		for ruleset in rulesets.iter_mut() {
			println!("DEBUG: Loading ruleset: {} ({})", ruleset.title.as_ref().unwrap_or(&"unnamed".to_string()), ruleset.ruleset_id.as_mut());
			all_rulesets.insert(ruleset.ruleset_id.clone(), ruleset.clone());
		}
	}

	if rulef.rules.is_some() {
		let mut rules = rulef.rules.unwrap();
		for rule in rules.iter_mut() {
			println!("DEBUG: Loading rule: {} ({})", rule.title, rule.rule_id);
			if rule.regex.is_some() {
				let rule_regex = rule.regex.as_mut().unwrap();
				let mut regex_builder = RegexBuilder::new(&rule_regex.pattern);
				if rule_regex.case_insensitive {
					regex_builder.case_insensitive(true);
				}
				let regex = regex_builder.build();
				if regex.is_ok() {
					rule_regex.regex = regex.ok();
				} else {
					println!("ERROR: Invalid regex for rule_id {}: {}", rule.rule_id, regex.err().unwrap());
					continue;
				}

			}
			all_rules.insert(rule.rule_id.clone(), rule.clone());
		}
	}
}
