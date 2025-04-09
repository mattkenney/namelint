mod parse_rules;
mod schema;
mod structs;
mod load;

use std::{collections::HashMap, fs};

use clap::{arg, Command};

use parse_rules::parse_rules;
use structs::{Rule, RuleSet};
use schema::{must_load_validator, SchemaType};

fn main() {

	let mut command = Command::new("exam-cli")
		.version("1.0")
		.about("Check that the examples for each rule are valid");

	command = command.arg(arg!(--rules <FILE> "Specify rule definitions to load")
		.required(true)
		.action(clap::ArgAction::Append)
		.value_parser(clap::value_parser!(String)));

	command = command.arg(
		arg!(<rule_id>... "specific rules to check (defaults to all loaded rules)")
			.required(false)
			.help("Rule(s) to checks")
			.trailing_var_arg(true),
	);

	let binding = command.get_matches();

	let validator = must_load_validator(SchemaType::Rule);

	let mut all_rules: HashMap<String, Rule> = HashMap::new();
	let mut all_rulesets: HashMap<String, RuleSet> = HashMap::new();

	let mut bad_file_count = 0;

	let mut custom_rules = binding.get_many::<String>("rules")
		.unwrap()
		.map(|s| s.as_str());

	while let Some(custom_rule) = custom_rules.next() {
		let body = fs::read_to_string(custom_rule);
		if body.is_err() {
			println!("ERROR: Unable to read rule file {}: {}", custom_rule, body.as_ref().err().unwrap());
			bad_file_count += 1;
		}
		let body = body.unwrap();

		parse_rules(&body, custom_rule, &validator, &mut all_rules, &mut all_rulesets);
	}

	println!("INFO: loaded {} rules", all_rules.len());

	let rule_ids: Vec<String>;
	if binding.contains_id("rule_id") {
		rule_ids = binding
			.get_many::<String>("rule_id")
			.unwrap()
			.map(|s| s.as_str())
			.map(|s| s.to_string())
			.collect();
	} else {
		rule_ids = all_rules.keys().map(|s| s.to_string()).collect();
	}

	println!("INFO: checking {} rules", rule_ids.len());

	let mut error_count = 0;
	for rule_id in rule_ids.iter() {
		let rule = all_rules.get(rule_id);
		if rule.is_none() {
			println!("ERROR: Rule {} not found", rule_id);
			error_count += 1;
			continue;
		}

		let rule = rule.unwrap();

		if rule.examples.is_none() {
			println!("ERROR: Rule {} has no examples (1)", rule_id);
			error_count += 1;
			continue;
		}
		let examples = rule.examples.as_ref().unwrap();
		if examples.is_empty() {
			println!("ERROR: Rule {} has no examples (2)", rule_id);
			error_count += 1;
			continue;
		}

		if rule.regex.is_none() {
			println!("ERROR: Rule {} has no regex", rule_id);
			error_count += 1;
			continue;
		}

		let regex_data = rule.regex.as_ref().unwrap();
		if regex_data.regex.is_none() {
			println!("ERROR: Rule {} has invalid regex", rule_id);
			error_count += 1;
			continue;
		}
		let regex = regex_data.regex.as_ref().unwrap();
		let expect = regex_data.expect == "matches";

		for (index, example) in examples.iter().enumerate() {
			let value = &example.value;
			let have = &example.expect == "pass";
			let mut regex_result = regex.is_match(value);
			if !expect {
				regex_result = !regex_result;
			}
			if have != regex_result {
				println!(
					"ERROR: Rule {} example #{} failed!",
					rule_id,
					index + 1, // Adding 1 to make it 1-based index
				);
				error_count += 1;
			} else {
				println!(
					"INFO: Rule {} example #{} passed!",
					rule_id,
					index + 1, // Adding 1 to make it 1-based index
				);
			}
		}
	}

	if error_count > 0 {
		println!("ERROR: {} examples with errors", error_count);
		std::process::exit(1);
	}

	if bad_file_count > 0 {
		println!("ERROR: {} rule files with errors", bad_file_count);
		std::process::exit(1);
	}
	println!("INFO: Success!");
}
