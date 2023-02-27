mod parsing;
use parsing::parse_object;

use std::env;

fn main() {
	env::set_var("RUST_BACKTRACE", "1");

    let val = std::fs::read_to_string("test.json").unwrap();

	let obj = parse_object(val.as_str());
	println!("{}", obj.to_string());
}
