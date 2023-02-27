mod parsing;
use parsing::parse_object;

mod formatting;
use formatting::format;

use std::env;

fn main() {
	env::set_var("RUST_BACKTRACE", "1");

    let val = std::fs::read_to_string("test.json").unwrap();

	let obj = parse_object(val.as_str());
	println!("{}", format(obj, 1, false));
}
