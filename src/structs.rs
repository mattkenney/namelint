use regex::Regex;
use serde;

#[derive(serde::Deserialize,Debug, Clone)]
pub struct RuleRegex {
	pub pattern: String,
	#[serde(default)]
	pub case_insensitive: bool,
	#[serde(skip)]
	pub regex: Option<Regex>,
	pub expect: String,
}

#[derive(serde::Deserialize,Debug, Clone)]
pub struct RuleExample {
	pub value: String,
	pub expect: String,
	pub notes: Option<String>,
}

#[derive(serde::Deserialize,Debug, Clone)]
pub struct Rule {
	//description: String,
	pub rule_id: String,
	pub regex: Option<RuleRegex>,
	pub title: String,
	pub examples: Option<Vec<RuleExample>>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct RuleSet {
	pub ruleset_id: String,
	//rules: Vec<String>,
	//rulesets: Vec<String>,
	pub title: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct RuleFile {
	pub rules: Option<Vec<Rule>>,
	pub rulesets: Option<Vec<RuleSet>>,
}
