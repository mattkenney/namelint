
fn test_name(_rules: &Vec<Rule>, name: &OsString) -> core::result::Result<(), String>{
	let name_str = std::str::from_utf8(name.as_encoded_bytes());
	if name_str.is_err() {
		return Err("ERROR: Path is not UTF-8".to_string());
	}
	/*
	let name_str = name_str.unwrap();
	for (rule_id, rule_regex) in rules.iter() {
		let regex = rule_regex.regex.as_ref().unwrap();
		let found = regex.is_match(name_str);
		println!("Checking rule {} on {}: {}", rule_id, name_str, found);
		if found && rule_regex.result == "fail" {
			return Err(format!("Matched rule {}", rule_id));
		} else if found == false && rule_regex.result == "pass" {
			return Err(format!("Did not match rule {}", rule_id));
		}
	}
	*/
	return Ok(());
}

fn process_file(rules: &Vec<Rule>, path: &Path) {
	let file_name = path.file_name();
	if file_name.is_none() {
		println!("ERROR: Unable to get file name for {:?}", path);
		return;
	}
	let file_name = file_name.unwrap();
	let results = test_name(rules, &file_name.to_os_string());
	if results.is_err() {
		println!("ERROR: Invalid file {}: {}", path.to_string_lossy(), results.unwrap_err());
	}
/*
			if rules::nfc::nfc(entry_path.to_string_lossy()) == false {
				println!("WARNING: Non-NFC filename: {:?}", &entry_path);
			}
*/
}
