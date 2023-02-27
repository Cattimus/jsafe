mod json;
mod parsing;
mod formatting;

use std::env;

fn main() {
	env::set_var("RUST_BACKTRACE", "1");

    let val = std::fs::read_to_string("test.json").unwrap();

	let obj = parsing::parse_object(val.as_str());
	println!("{}", formatting::format(obj, 1, false));
}
