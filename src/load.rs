use serde_json;
use serde_yaml;
//use std::fs;

pub fn must_load_json(data: &str, src: &str) -> serde_json::Value {
	let value = serde_json::from_str(data);
	if value.is_err() {
		println!(
			"ERROR: unable to parse json '{}': {}",
			src,
			value.err().unwrap()
		);
		std::process::exit(1);
	}
	value.unwrap()
}

pub fn must_load_yaml(data: &str, src: &str) -> serde_yaml::Value {
	let value = serde_yaml::from_str(data);
	if value.is_err() {
		println!(
			"ERROR: unable to parse yaml '{}': {}",
			src,
			value.err().unwrap()
		);
		std::process::exit(1);
	}
	value.unwrap()
}

pub fn must_yaml_to_json(yaml: &serde_yaml::Value, src: &str) -> serde_json::Value {
	let json_str = serde_json::to_string(&yaml);
	if json_str.is_err() {
		println!(
			"ERROR: unable to convert yaml value to json string '{}': {}",
			src, json_str.err().unwrap()
		);
		std::process::exit(1);
	}
	let json_str = json_str.unwrap();
	let value = serde_json::from_str(&json_str);
	if value.is_err() {
		println!(
			"ERROR: unable to parse json '{}': {}", src,
			value.err().unwrap()
		);
		std::process::exit(1);
	}
	value.unwrap()
}

pub fn must_load_data(data: &str, data_type: &str, src: &str) -> serde_json::Value {
	let ret_val: serde_json::Value;
    if data_type == "yaml" {
		let yaml_value = must_load_yaml(data, src);
		ret_val = must_yaml_to_json(&yaml_value, src);
	} else {
		ret_val = must_load_json(data, src);
	}
	ret_val
}
/*
pub fn load_yaml_as_json(data: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(data)?;
    let json_str = serde_json::to_string(&yaml_value)?;
    let json_value = serde_json::from_str(&json_str)?;
    Ok(json_value)
}

pub fn load_file(file_name: &str) -> Result<serde_json::Value, Box<dyn Error>> {
	let data = fs::read_to_string(file_name)?;
	if file_name.ends_with("yml") || file_name.ends_with("yaml") {
		return load_yaml_as_json(&data);
	}
	let json_value = serde_json::from_str(&data)?;
	Ok(json_value)
}

pub fn must_load_file(file_name: &str) -> String {
	let data = fs::read_to_string(file_name);
	if data.is_err() {
		println!("ERROR: unable to read file '{}': {}", file_name, data.err().unwrap());
		std::process::exit(1);
	}
	data.unwrap()
}

*/
