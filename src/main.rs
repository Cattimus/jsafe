mod json;
use json::Value;

fn main() {
	let mut root = Value::obj();
	root.pre_alloc(1000000);
	for i in 0..1000000 {
		root[i.to_string().as_str()] = i.into();
	}
}
