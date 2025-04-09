use jsonschema::{self, Validator};
use crate::load::{must_load_data, must_load_file};
use std::fmt;

const DEFAULT_RULE_SCHEMA: &'static str = include_str!("../docs/namelint-rule-schema.yaml");
const DEFAULT_CONFIG_SCHEMA: &'static str = include_str!("../docs/namelint-config-schema.yaml");

pub enum SchemaType {
	Rule,
	Config,
}

impl SchemaType {
	pub fn as_str(&self) -> &str {
		match self {
			SchemaType::Rule => "rule",
			SchemaType::Config => "config",
		}
	}
}

impl fmt::Display for SchemaType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			SchemaType::Rule => write!(f, "rule"),
			SchemaType::Config => write!(f, "config"),
		}
	}
}

pub fn must_load_validator(schema_type: SchemaType) -> Validator {

	let schema_str = match schema_type {
		SchemaType::Rule => DEFAULT_RULE_SCHEMA.to_string(),
		SchemaType::Config =>  DEFAULT_CONFIG_SCHEMA.to_string()
	};

	let schema = must_load_data(&schema_str, "yaml", schema_type.as_str());

	let validator = jsonschema::validator_for(&schema);
	if validator.is_err() {
		println!("ERROR: unable to load schema validator '{}': {}", schema_type, validator.err().unwrap());
		std::process::exit(1);
	}
	validator.unwrap()
}

