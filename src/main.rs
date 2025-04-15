#![allow(unused)]

mod safe_name;
mod load;
mod load_dir;
mod parse_config;
mod parse_rules;
mod run_lint;
mod schema;
mod structs;
mod rules;
mod regex_rule;

use std::{collections::HashMap, ffi::OsString, fs::{self}};
use clap::{arg, Command};
use include_dir::{include_dir, Dir};
use std::collections::HashSet;

use load_dir::load_dir;
use parse_config::parse_config;
use parse_rules::parse_rules;
use run_lint::run_lint;
use schema::{must_load_validator, SchemaType};
use structs::{ConfigFile, FileData, Lint, RuleFn, RuleSet};

static RULES_DIR: Dir = include_dir!("./rules");

const DEFAULT_CONFIG: &'static str = include_str!("../config/default.yaml");

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

	command = command.arg(arg!(<path>... "paths to check")
		.required(false)
		.help("Path(s) to checks")
		.trailing_var_arg(true));

	let binding = command.get_matches();
	//LATER: convert to log level let verbose = binding.get_count("verbose");
	let rule_validator = must_load_validator(SchemaType::Rule);

	let mut all_rules: HashMap<String, RuleFn> = HashMap::new();
	let mut all_rulesets: HashMap<String, RuleSet> = HashMap::new();

	for rule_file in RULES_DIR.find("*.yaml").unwrap() {
		let body = RULES_DIR.get_file(rule_file.path()).unwrap().contents_utf8().unwrap();
		let src = rule_file.path().display().to_string();
		parse_rules(body, &src, &rule_validator, &mut all_rules, &mut all_rulesets);
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
			parse_rules(&body, custom_rule, &rule_validator, &mut all_rules, &mut all_rulesets);
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

	let config_validator = must_load_validator(SchemaType::Config);
	let mut config:ConfigFile;

	let config_file = binding.get_one::<String>("config");
	if config_file.is_some() {
		let config_file = config_file.unwrap();
		println!("DEBUG: loading config file {}", config_file);
		let config_file_str = fs::read_to_string(config_file);
		if config_file_str.is_err() {
			println!("ERROR: Unable to read config file {}: {}", config_file, config_file_str.err().unwrap());
			std::process::exit(2);
		}
		let config_str = config_file_str.unwrap();
		config = parse_config(&config_str, &config_file, &config_validator);
	} else {
		println!("DEBUG: using default config");
		let config_str = DEFAULT_CONFIG.to_string();
		config = parse_config(&config_str, "<default>", &config_validator);
	}

	//LATER: load rules/rulesets from config...

	let dirs:Vec<String>;

	if binding.contains_id("path") {
		println!("DEBUG: using directories from the command line");
		dirs = binding.get_many::<String>("path")
			.unwrap()
			.map(|s| s.to_string())
			.collect();
	} else if config.dirs.is_some() {
		println!("DEBUG: using directories from config file");
		dirs = config.dirs.unwrap();
	} else {
		println!("DEBUG: using current directory");
		dirs = vec![".".to_string()];
	}

	if dirs.len() == 0 {
		println!("ERROR: no directories to check");
		std::process::exit(6);
	}

	let ignore_dirs: HashSet<OsString>;
	if config.ignore_dirs.is_some() {
		println!("DEBUG: using ignore directories from config file");
		ignore_dirs = config.ignore_dirs.unwrap().into_iter().map(OsString::from).collect();
	} else {
		println!("DEBUG: no ignore directories");
		ignore_dirs = HashSet::new();
	}

	println!("DEBUG: scanning {} listed directories", dirs.len());

	let mut files:Vec<FileData> = Vec::new();
	for dir in dirs.iter() {
		let dir = dir.to_string();
		if load_dir(dir.clone(), &ignore_dirs, &mut files) == false {
			println!("ERROR: unable to load directory {}", dir);
			std::process::exit(5);
		}
	}
	println!("DEBUG: directory scan complete - checking {} files", files.len());

	for file in files.iter() {
		println!("DEBUG: file '{}'", file.lintpath);
	}

	for (index, cfg_lint) in config.lints.iter_mut().enumerate() {
		let lint:Lint = Lint {
			name: cfg_lint.name.clone().unwrap_or_else(|| format!("#{}", index)),
			paths: cfg_lint.paths.clone(),
			rules: cfg_lint.rules.clone(),
		};
		println!("DEBUG: applying lint {}", lint.name);
		run_lint(&lint, &mut files, &all_rules, &all_rulesets);
	}

	for file in files.iter_mut() {
		if file.passed.len() == 0  && file.failed.len() == 0 {
			file.failed.push("not-linted".to_string());
		}
	}

	let mut pass_count = 0;
	let mut fail_count = 0;

	for file in files.iter() {
		if file.failed.len() > 0 {
			println!("ERROR: file '{}' failed: {}", file.lintpath, file.failed.join(", "));
			fail_count += 1;
		} else {
			pass_count += 1;
		}
	}

	println!("INFO: {} files passed, {} files failed", pass_count, fail_count);
	if fail_count > 0 {
		std::process::exit(1);
	}

}


