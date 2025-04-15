use jsonschema::{self, Retrieve, Uri, ValidationError, Validator};
use serde_json::Value;
use std::{collections::HashMap, fmt};

use crate::load::must_load_data;

const DEFAULT_RULE_SCHEMA: &'static str = include_str!("../docs/namelint-rule-schema.yaml");
const DEFAULT_CONFIG_SCHEMA: &'static str = include_str!("../docs/namelint-config-schema.yaml");

#[derive(PartialEq)]
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
	/*
	pub fn from_str(s: &str) -> Option<Self> {
		match s {
			"rule" => Some(SchemaType::Rule),
			"config" => Some(SchemaType::Config),
			_ => None,
		}
	}

	pub fn get_validator(&self) -> Validator {
		match self {
			SchemaType::Rule => must_load_validator(SchemaType::Rule),
			SchemaType::Config => must_load_validator(SchemaType::Config),
		}
	}
	*/

	pub fn eq(&self, other: &SchemaType) -> bool {
		matches!(
			(self, other),
			(SchemaType::Rule, SchemaType::Rule) | (SchemaType::Config, SchemaType::Config)
		)
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

struct RefRetriever {
	schemas: HashMap<String, Value>,
}


impl Retrieve for RefRetriever {
	fn retrieve(&self, uri: &Uri<String>) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
		self.schemas
			.get(uri.as_str())
			.cloned()
			.ok_or_else(|| format!("ERROF ref$ schema not found: {uri}").into())
	}
}

fn build_ref_retriever() -> RefRetriever {
	let mut retriever = RefRetriever {
		schemas: HashMap::new(),
	};

	retriever.schemas.insert(
		"https://www.namelint.dev/namelint-rule-schema.json".to_string(),
		must_load_data(DEFAULT_RULE_SCHEMA, "yaml", "ref"),
	);

	retriever
}

pub fn must_load_validator(schema_type: SchemaType) -> Validator {

	let schema_str = match schema_type {
		SchemaType::Rule => DEFAULT_RULE_SCHEMA.to_string(),
		SchemaType::Config =>  DEFAULT_CONFIG_SCHEMA.to_string()
	};

	let schema = must_load_data(&schema_str, "yaml", schema_type.as_str());

	let validator: Result<Validator, ValidationError>;
	if schema_type == SchemaType::Config {
		// Add the rule schema to the config schema
		//let rule_schema = must_load_data(DEFAULT_RULE_SCHEMA, "yaml", "built-in");
		//let retriever = InMemoryRetriever{ schema: rule_schema };
		validator = jsonschema::options()
			.with_retriever(build_ref_retriever())
			.build(&schema);
	} else {
		validator = jsonschema::validator_for(&schema);
	}

	if validator.is_err() {
		println!("ERROR: unable to load schema validator '{}': {}", schema_type, validator.err().unwrap());
		std::process::exit(1);
	}
	validator.unwrap()
}

