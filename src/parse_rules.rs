use std::collections::HashMap;
use regex::RegexBuilder;

use crate::load::{must_load_yaml, must_yaml_to_json};
use crate::structs::{Rule, RuleFile, RuleSet};

pub fn parse_rules(body: &str, src: &str, validator: &jsonschema::Validator, all_rules: &mut HashMap<String, Rule>, all_rulesets: &mut HashMap<String, RuleSet>) {
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
