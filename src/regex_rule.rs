use regex::{Regex, RegexBuilder};

use crate::structs::{ Rule, RuleFn };

pub fn build_regex_rule_fn<'a>(the_rule: &Rule) -> RuleFn<'a> {
	let rule_id: &'static str = Box::leak(the_rule.rule_id.clone().into_boxed_str());
	let rregex = the_rule.regex.clone();
	if rregex.is_none() {
		println!("ERROR: Rule {} has no regex", rule_id);
		std::process::exit(9);
	}
	let rregex = rregex.unwrap();
	let pattern = rregex.pattern;
	let case_insensitive = rregex.case_insensitive;
	let matches: bool;
	if rregex.expect == "match" {
		matches = true;
	} else if rregex.expect == "nonmatch" {
		matches = false;
	} else {
		println!("ERROR: Rule {} has invalid expect value: '{}'", rule_id, rregex.expect);
		std::process::exit(10);
	}

	let mut regex_builder = RegexBuilder::new(&pattern);
	if case_insensitive {
		regex_builder.case_insensitive(true);
	}
	let regex = regex_builder.build();
	if regex.is_err() {
		let err = regex.as_ref().err().unwrap();
		println!("ERROR: Invalid regex for rule_id {}: {}", rule_id, err);
	}

	let regex: &'static Regex = Box::leak(Box::new(regex.unwrap()));

	Box::new(move |test: &str| {
		if regex.is_match(test) == matches {
			println!("DEBUG: Rule {} success for: '{}'", rule_id, test);
			return true;
		} else {
			println!("DEBUG: Rule {} failed for : '{}'", rule_id, test);
			return false;
		}
	})
}

