use std::{ffi::OsString, path::{Path, PathBuf}};

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

#[derive(serde::Deserialize, Debug, Clone)]
pub struct FileData {
	pub path: PathBuf,
	pub passed: Vec<String>,
	pub failed: Vec<String>,
	pub fatal: bool,			// if true, skip all other rules
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ConfigTest {
	paths: Vec<String>,
	pub rules: Vec<String>,
	pub rulesets: Vec<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ConfigFile{
	pub dirs: Vec<String>,
	pub tests: Vec<ConfigTest>,
}
