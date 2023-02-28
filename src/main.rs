mod json;
use json::formatting;
use json::Value;


fn main() {
	let mut root = Value::obj();
	root.alloc(1000000);
	
	for i in 0..1000000 {
		root[i.to_string().as_str()] = Value::Number(i as f64);
	}

	//println!("{}", formatting::format(root, 2));
}
