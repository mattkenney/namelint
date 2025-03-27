use unicode_normalization::is_nfc;

pub fn nfc(input: &str) -> bool {

	return is_nfc(input);
}
