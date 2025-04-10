use std::collections::HashMap;

use crate::structs::{ConfigLint, FileData, Rule, RuleSet};

pub fn run_lint(lint: &ConfigLint, files: &mut Vec<FileData>, all_rules: &HashMap<String, Rule>, _all_rulesets: &HashMap<String, RuleSet>) {


	let mut rules: Vec<Rule> = Vec::new();

	let lint_rules = &lint.rules;
	if lint_rules.is_some() {
		let lint_rules = lint_rules.as_ref().unwrap();

		for rule_id in lint_rules.iter() {

			let the_rule = all_rules.get(rule_id);
			if the_rule.is_none() {
				println!("ERROR: unknown rule_id '{}'", rule_id);
				std::process::exit(7);
			}
			let the_rule = the_rule.unwrap();
			rules.push(the_rule.clone());
		}
	}

	// LATER: load rules from rulesets
	if rules.len() == 0 {
		println!("WARNING: no rules for {}", lint.name.clone().unwrap());
		return;
	}

	for rule in rules.iter() {

		if rule.regex.is_none() {
			println!("ERROR: Rule {} has no regex", rule.rule_id);
			continue;
		}

		let regex_data = rule.regex.as_ref().unwrap();
		if regex_data.regex.is_none() {
			println!("ERROR: Rule {} has invalid regex", rule.rule_id);
			continue;
		}
		let regex = regex_data.regex.as_ref().unwrap();
		let expect = regex_data.expect == "match";

		for file in files.iter_mut() {
			let value = &file.file_name;
			let result = regex.is_match(value);
			if result != expect {
				println!("DEBUG: failed match for '{}': rule={} expect={}, got={}", value, regex_data.pattern, expect, result);
				file.failed.push(rule.rule_id.clone());
			} else {
				file.passed.push(rule.rule_id.clone());
			}
		}
	}

}
