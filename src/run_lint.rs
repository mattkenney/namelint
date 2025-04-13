use std::collections::HashMap;

use crate::structs::{Lint, FileData, RuleFn, RuleSet};

pub fn run_lint(lint: &Lint, files: &mut Vec<FileData>, all_rules: &HashMap<String, RuleFn>, _all_rulesets: &HashMap<String, RuleSet>) {


	// LATER: load rules from rulesets too
	let lint_rules = &lint.rules;
	if lint_rules.is_some() {
		let lint_rules = lint_rules.as_ref().unwrap();

		for rule_id in lint_rules.iter() {

			let the_rule = all_rules.get(rule_id);
			if the_rule.is_none() {
				println!("ERROR: lint {} has unknown rule_id '{}'", lint.name, rule_id);
				std::process::exit(7);
			}
			let rule_fn = the_rule.unwrap();


			for file in files.iter_mut() {
				let value = &file.file_name;
				let result = rule_fn(value);
				if result {
					file.passed.push(rule_id.clone());
				} else {
					println!("DEBUG: failed match for '{}': rule={}", file.file_name, rule_id);
					file.failed.push(rule_id.clone());
				}
			}
		}
	}

}
