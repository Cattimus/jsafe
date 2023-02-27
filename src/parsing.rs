use crate::json;
use json::Value;

fn remove_whitespace(input: &str) -> String {
	let mut to_return = String::from("");

	let mut in_quote = false;
	let mut prev = ' ';
	for i in 0..input.len() {
		let cur = input.chars().nth(i).unwrap();
		match cur {
			c if c.is_whitespace() => {
				if in_quote {
					to_return.push(cur);
				} else {
					continue;
				}
			}

			//make sure we know if we're in a quote or not
			'"' => {
				if prev != '\\' {
					in_quote = !in_quote;
				}

				to_return.push(cur);
			}

			_ => {to_return.push(cur);}
		}

		prev = cur;
	}

	return to_return;
}

pub fn parse_value(value: &str) -> Value {
	let first = value.chars().nth(0).unwrap();
	match first {
		'"' => {
			let value = &value[1..value.len()-1];
			value.into()
		}

		'{' => {
			parse_object(value)
		}

		'[' => {
			parse_array(value)
		}

		't' | 'f' => {
			Value::Bool(value == "true")
		}

		'n' => {
			if value == "null" {
				return Value::Null
			}

			println!("Value is n but not null: {}", value);
			Value::Invalid
		}

		c if c.is_numeric() => {
			Value::Number(value.parse().unwrap())
		}

		_ => {
			println!("Invalid value: {}", value);
			Value::Invalid
		}
	}
}

fn parse_keypair(keypair: &str) -> (&str, Value) {

	//split keypair value
	let val = keypair.split_at(keypair.find(':').unwrap());
	let key = val.0;
	let value = &val.1[1..];

	//remove quotes from key value
	let key = &key[1..key.len()-1];

	//parse value
	(key, parse_value(value))
}

fn get_splits(input: &str) -> Option<Vec<usize>> {
	//get split indexes for current object
	let mut in_quote = false;
	let mut obj_level = 0;
	let mut arr_level = 0;
	let mut split_indexes: Vec<usize> = Vec::new();
	let mut prev: char = ' ';
	for i in 0..input.len() {
		let cur = input.chars().nth(i).unwrap();

		match cur {
			//quote start/end
			'"' => {
				if prev != '\\' {
					in_quote = !in_quote;
				}
			}
			
			//object start
			'{' => {
				if !in_quote {
					obj_level += 1;
				}
			}
			
			//object end
			'}' => {
				if !in_quote {
					obj_level -= 1;
				}
			}

			//array start
			'[' => {
				if !in_quote {
					arr_level += 1;
				}
			}

			//array end
			']' => {
				if !in_quote {
					arr_level -= 1;
				}
			}

			//new split index for the current object
			',' => {
				if (obj_level + arr_level == 0) && !in_quote {
					split_indexes.push(i);
				}
			}

			_ => {}
		}

		prev = cur;
	}

	if obj_level != 0 || arr_level != 0 || in_quote {
		println!("JSON: Split was unable to properly parse value.");
		println!("Obj level: {}", obj_level);
		println!("Arr level: {}", arr_level);
		println!("In quote: {}", in_quote);
		println!("Input value: {}", input);
		return None;
	}

	return Some(split_indexes);
}

pub fn parse_object(input: &str) -> Value {
	let mut to_return = Value::obj();

	//we know if the first character isn't an opening bracket, our json object is invalid
	if input.chars().nth(0).unwrap() != '{' {
		println!("Object does not start with {{");
		return Value::Invalid;
	}

	//get split indexes
	let trimmed = remove_whitespace(input);
	let trimmed_len = trimmed.len();
	let trimmed = &trimmed[1..trimmed_len-1];

	let split_indexes = get_splits(trimmed);
	if split_indexes.is_none() {
		return Value::Invalid;
	}
	let split_indexes = split_indexes.unwrap();

	//split along all key:value pairs
	let mut prev_index = 0;
	let mut keypairs: Vec<&str> = Vec::new();
	for i in split_indexes {
		let first = trimmed.split_at(prev_index).1;
		let second = first.split_at(i - prev_index);
		let data = second.0;

		keypairs.push(&data);
		prev_index = i;
	}
	keypairs.push(trimmed.split_at(prev_index).1);

	//remove ,
	for i in 1..keypairs.len() {
		keypairs[i] = &keypairs[i][1..];
	}

	//parse keypairs
	for pair in keypairs {
		let result = parse_keypair(pair);

		//some part of the data is invalid
		if let Value::Invalid = result.1 {
			return Value::Invalid;
		}

		//assign value
		to_return[result.0] = result.1;
	}

	return to_return;
}

pub fn parse_array(input: &str) -> Value{
	let mut to_return = Value::arr();
	
	//invalid array
	if input.chars().nth(0).unwrap() != '[' {
		println!("JSON: first character of array is not [.");
		return Value::Invalid;
	}

	//get split indexes
	let trimmed = remove_whitespace(input);
	let trimmed_len = trimmed.len();
	let trimmed = &trimmed[1..trimmed_len-1];

	let split_indexes = get_splits(trimmed);
	if split_indexes.is_none() {
		return Value::Invalid;
	}
	let split_indexes = split_indexes.unwrap();

	//split into value strings
	let mut prev_index = 0;
	let mut values: Vec<&str> = Vec::new();
	for i in split_indexes {
		let temp = trimmed.split_at(i);
		values.push(&temp.0);
		prev_index = i;
	}
	values.push(trimmed.split_at(prev_index).1);

	//remove ,
	for i in 1..values.len() {
		values[i] = &values[i][1..];
	}

	//parse values
	for value in values {
		let text = remove_whitespace(value);
		let val = parse_value(text.as_str());

		if let Value::Invalid = val {
			return Value::Invalid;
		}

		to_return.append(val);
	}

	return to_return;
}