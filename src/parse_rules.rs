use std::collections::HashMap;

use crate::load::{must_load_yaml, must_yaml_to_json};
use crate::structs::{RuleFile, RuleFn, RuleSet};
use crate::regex_rule::build_regex_rule_fn;

pub fn parse_rules(body: &str, src: &str, validator: &jsonschema::Validator, all_rules: &mut HashMap<String, RuleFn>, all_rulesets: &mut HashMap<String, RuleSet>) {
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
		let rules = rulef.rules.unwrap();
		for rule in rules.iter() {
			println!("DEBUG: Loading rule: {} ({})", rule.title, rule.rule_id);
			let tmp_rule = rule.clone();
			all_rules.insert(rule.rule_id.clone(), build_regex_rule_fn(&tmp_rule));
		}
	}
}
