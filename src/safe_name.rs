use std::ffi::OsString;

use phf::phf_map;

pub fn safe_byte(b: u8) -> String {
	if (b >= 0x30 && b <= 0x39) || (b >= 0x41 && b <= 0x5A) || (b >= 0x61 && b <= 0x7A) || b == 0x2E {
		// alphanumeric or dot
		return (b as char).to_string();
	}
	// non-alphanumeric
	format!(":0x{:02X}:", b)
}

pub fn safe_bytes(input: Vec<u8>) -> String {
	let output = input
		.iter()
		.map(|b| safe_byte(*b))
		.collect::<String>();

	return output;
}

// should only be called if into_string() fails
pub fn safe_bad_os_string(input: &OsString) -> String {
	// may not be valid UTF-8
	let bytes: Vec<u8> = input.clone().into_encoded_bytes();
	return safe_bytes(bytes);
}

pub fn safe_os_string(input: &OsString) -> String {

	let string = input.clone().into_string();
	if string.is_ok() {
		return safe_string(&string.unwrap());
	}
	return safe_bad_os_string(input);
}


static CHAR_MAP: phf::Map<char, &'static str> = phf_map! {
	' ' => ":space:",
	':' => ":colon:",
	'/' => ":slash:",
	'\\' => ":backslash:",
	'<' => ":lt:",
	'>' => ":gt:",
	'?' => ":question:",
	'*' => ":star:",
	'|' => ":pipe:",
	'"' => ":dquote:",
	'\'' => ":squote:",
	'`' => ":backtick:",
	'!' => ":bang:",
	'@' => ":at:",
	'#' => ":hash:",
	'$' => ":dollar:",
	'%' => ":percent:",
	'^' => ":caret:",
	'&' => ":amp:",
	'(' => ":lparen:",
	')' => ":rparen:",
	'[' => ":lbracket:",
	']' => ":rbracket:",
	'{' => ":lbrace:",
	'}' => ":rbrace:",
	'=' => ":eq:",
	'+' => ":plus:",
	'-' => ":minus:",
	'_' => ":underscore:",
	'~' => ":tilde:",
	',' => ":comma:",
	';' => ":semicolon:",
};

pub fn safe_char(input: char) -> String {
	if (input >= '0' && input <= '9') || (input >= 'A' && input <= 'Z') || (input >= 'a' && input <= 'z') || input == '.' {
		// alphanumeric or dot
		return input.to_string();
	}
	if let Some(replacement) = CHAR_MAP.get(&input) {
		return replacement.to_string();
	}
	if input <= '~' {
		// non-alphanumeric
		return format!(":0x{:02X}:", input as u8);
	}
	return format!(":U+0x{:04X}:", input as u32);
}

pub fn safe_string(input: &str) -> String {
	let output = input
		.chars()
		.map(|c| safe_char(c))
		.collect::<String>();

	return output;
}
