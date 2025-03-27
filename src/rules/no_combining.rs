

pub fn no_combining(input: &str) -> bool {
	let mut output = String::new();
	for c in input.chars() {
		if c.is_combining_mark() {
			return false;
		}
	}
	return true;
}
