mod schema;
mod load;

use clap::{arg, Command};
use schema::{must_load_validator, SchemaType};
use load::load_file;

fn main() {

	let mut command = Command::new("validate")
		.version("1.0")
		.about("Check file names for security, compatibility, best practices & standards.");

	command = command.arg(
		arg!(<path>... "paths to check")
			.required(true)
			.help("Path(s) to checks")
			.trailing_var_arg(true),
	);

	let binding = command.get_matches();
	let validator = must_load_validator(SchemaType::Rule);

	let mut error_count = 0;
	let mut bad_file_count = 0;

	for path in binding.get_many::<String>("path").unwrap() {

		let data = load_file(path);
		if data.is_err() {
			println!("ERROR: Unable to read file {}: {}", path, data.err().unwrap());
			error_count += 1;
			bad_file_count += 1;
			continue;
		}
		let data = data.unwrap();

		let errors =validator.iter_errors(&data);
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
