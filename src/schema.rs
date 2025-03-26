use jsonschema::{self, Validator};
use crate::load::{must_load_data, must_load_file};

const DEFAULT_SCHEMA: &'static str = include_str!("../docs/namelint-schema.yaml");

pub fn must_load_validator(file_name: Option<&String>) -> Validator {
	let schema_str: String;
	let mut schema_type = "json";
	let schema_src: &str;

	if file_name.is_none() {
		println!("INFO: Using default schema");
		schema_str = DEFAULT_SCHEMA.to_string();
		schema_type = "yaml";
		schema_src = &"built-in schema";
	} else {
		let schema_file = file_name.unwrap();
		println!("INFO: Using schema file {}", schema_file);
		schema_str = must_load_file(schema_file);
		if schema_file.ends_with("yml") || schema_file.ends_with("yaml") {
			schema_type = "yaml";
		}
		schema_src = schema_file;
	}
	let schema = must_load_data(&schema_str, schema_type, schema_src);

	let validator = jsonschema::validator_for(&schema);
	if validator.is_err() {
		println!("ERROR: unable to load schema validator '{}': {}", schema_src, validator.err().unwrap());
		std::process::exit(1);
	}
	validator.unwrap()
}

