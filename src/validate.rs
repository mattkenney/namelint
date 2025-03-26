use clap::{arg, Command};
use serde_json;
use std::fs::{self};


fn main() {
	let default_schema: &'static str = include_str!("../docs/namelint-schema.yaml");

	let mut command = Command::new("namelint")
		.version("1.0")
		.about("Check file names for security, compatibility, best practices & standards.");

	command = command.arg(
		arg!(--schema <FILE> "Specify an alternate schema file")
			.required(false)
			.value_parser(clap::value_parser!(String)),
	);

	command = command.arg(
		arg!(--rules <FILE> "Specify additional rule definitions to load")
			.required(false)
			.num_args(1..)
			.value_parser(clap::value_parser!(String)),
	);

	command = command.arg(
		arg!(<path>... "paths to check")
			.required(true)
			.help("Path(s) to checks")
			.trailing_var_arg(true),
	);

	let binding = command.get_matches();
	let schema_file = binding.get_one::<String>("schema");
	let schema_str: String;
	let mut schema_type = "json";
	if schema_file.is_none() {
		println!("INFO: Using default schema");
		schema_str = default_schema.to_string();
		schema_type = "yaml";
	} else {
		let schema_file = schema_file.unwrap();
		println!("INFO: Using schema file {}", schema_file);
		schema_str = fs::read_to_string(schema_file).unwrap();
		if schema_file.ends_with("yml") || schema_file.ends_with("yaml") {	// LATER or sniff first char of file?
			schema_type = "yaml";
		}
	}

	let schema = if schema_type == "yaml" {
		let yaml_schema: Result<serde_yaml::Value, _> = serde_yaml::from_str(&schema_str);
		if yaml_schema.is_err() {
			println!("ERROR: unable to parse built-in schema: {}", yaml_schema.err().unwrap());
			std::process::exit(1);
		}
		let yaml_schema = yaml_schema.unwrap();
		serde_json::from_str(&serde_json::to_string(&yaml_schema).unwrap()).unwrap()
	} else {
		let schema = serde_json::from_str(&schema_str);
		if schema.is_err() {
			println!("ERROR: unable to parse schema '{}': {}", schema_file.unwrap(), schema.err().unwrap());
			std::process::exit(1);
		}
		schema.unwrap()
	};

	let validator = jsonschema::validator_for(&schema);
	if validator.is_err() {
		println!("ERROR: unable to load schema: {}", validator.err().unwrap());
		std::process::exit(1);
	}

	let validator = validator.unwrap();
	let mut error_count = 0;
	let mut bad_file_count = 0;

	for path in binding.get_many::<String>("path").unwrap() {
		let body = fs::read_to_string(path);
		if body.is_err() {
			println!("ERROR: Unable to read file {}: {}", path, body.err().unwrap());
			error_count += 1;
			bad_file_count += 1;
			continue;
		}
		let body = body.unwrap();
		if body.is_empty() {
			println!("WARNING: Skipping empty file '{}'", path);
			continue;
		}

		let yaml_data: Result<serde_yaml::Value, _> = serde_yaml::from_str(&body);
		if yaml_data.is_err() {
			println!("ERROR: unable to parse rules file '{}': {}", path, yaml_data.err().unwrap());
			error_count += 1;
			continue;
		}
		let yaml_data = yaml_data.unwrap();
		let rule_json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&yaml_data).unwrap()).unwrap();

		let errors =validator.iter_errors(&rule_json);
		let mut file_error_count = 0;

		for error in errors {
			file_error_count += 1;
			error_count += 1;
			println!("ERROR: {} ({})", error, error.instance_path);
		}
		if file_error_count == 0 {
			println!("INFO: No errors found in {}", path);
		} else {
			println!("ERROR: {} errors found in {}", file_error_count, path);
			bad_file_count += 1;
		}
	}
	if error_count > 0 {
		println!("ERROR: {} errors found in {} file(s)", error_count, bad_file_count);
		std::process::exit(1);
	}
	println!("INFO: Success!");
}
