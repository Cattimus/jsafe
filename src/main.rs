mod json;
use json::Value;

fn main() {
	let data = std::fs::read_to_string("test.json").expect("Cannot read input file.");

	let root = json::parsing::from_str(data.as_str());
	println!("{}", json::formatting::prettify(&root, 2));
}
