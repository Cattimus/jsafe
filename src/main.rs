mod json;
use json::formatting;


fn main() {
	let mut temp = json::Value::obj();
	temp["this"]["is"]["A"] = "test".into();
	temp["val"] = 15.21.into();
	temp["this"]["is"]["an"] = json::Value::arr();
	temp["this"]["is"]["an"].append("array".into());
	println!("{}", formatting::format(temp, 0));

	/*
    let val = std::fs::read_to_string("test.json").unwrap();

	let obj = parsing::parse_object(val.as_str());
	println!("{}", formatting::format(obj, 2));
	*/
}
