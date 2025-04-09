use crate::load::{must_load_yaml, must_yaml_to_json};
use crate::structs::ConfigFile;

pub fn parse_config(body: &str, src: &str, validator: &jsonschema::Validator) -> ConfigFile {
	let yaml_data = must_load_yaml(body, &src);
	let json_data = must_yaml_to_json(&yaml_data, &src);
	let is_valid = validator.validate(&json_data);
	if is_valid.is_err() {
		println!("ERROR: Invalid config file {}: {}", src, is_valid.err().unwrap());
		std::process::exit(3);
	}
	let config: ConfigFile = serde_yaml::from_str(body)
		.unwrap_or_else(|e| panic!("Unable to load config file {}: {}", &src, e));

	return config;
}
