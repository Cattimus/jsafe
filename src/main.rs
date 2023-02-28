mod json;
use json::formatting;
use json::Value;


fn main() {
	let mut root = Value::obj();
	
	for i in 0..1000000 {
		root[i.to_string().as_str()] = i.into();
	}
}
