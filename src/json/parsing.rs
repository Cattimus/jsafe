use crate::json::Value;

//Remove whitespace from a string (except for in quotes)
#[allow(unused)]
fn remove_whitespace(input: &str) -> String {
	let mut to_return = String::from("");

	//Flag that tells us if we're in a quote block or not
	let mut in_quote = false;
	//Previous character (so we can know if there was a \" or not)
	let mut prev = ' ';

	//Loop through every character of the array
	for c in input.chars() {
		match c {
			
			//Ignore whitespace unless it is in a quote block
			c if c.is_whitespace() => {
				if in_quote {
					to_return.push(c);
				} else {
					continue;
				}
			}

			//Make sure we know if we're in a quote or not
			'"' => {
				if prev != '\\' {
					in_quote = !in_quote;
				}

				to_return.push(c);
			}

			//Every other character is okay to push directly onto the string
			_ => {to_return.push(c);}
		}

		prev = c;
	}

	return to_return;
}

//Convert a string to a json::Value object
#[allow(unused)]
pub fn parse_value(value: &str) -> Value {

	//Check what the first character of the string is to see what the value should be
	let first = value.chars().nth(0).unwrap();
	match first {

		//Text
		'"' => {
			let value = &value[1..value.len()-1];
			value.into()
		}

		//Object
		'{' => {
			parse_object(value)
		}

		//Array
		'[' => {
			parse_array(value)
		}

		//Boolean (true/false)
		't' | 'f' => {
			Value::Bool(value == "true")
		}

		//Null
		'n' => {
			if value == "null" {
				return Value::Null
			}

			//Improperly formatted json will break in this way.
			println!("Value is n but not null: {}", value);
			Value::Invalid
		}

		//Number
		c if c.is_numeric() => {
			Value::Number(value.parse().unwrap())
		}

		//Miscellaneous invalid text
		_ => {
			println!("Invalid value: {}", value);
			Value::Invalid
		}
	}
}

//Parse a keypair into a (key, value) tuple
#[allow(unused)]
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

//Get a list of where we should split a string
#[allow(unused)]
fn get_splits(input: &str) -> Option<Vec<usize>> {

	//The list of indexes we should split on
	let mut split_indexes: Vec<usize> = Vec::new();

	//Are we in a quote block or not
	let mut in_quote = false;

	//How many objects deep are we
	let mut obj_level = 0;

	//How many arrays deep are we
	let mut arr_level = 0;
	
	//Previous character (for '\"')
	let mut prev: char = ' ';

	//Loop through every character in the string. We need the index to push to the list.
	for i in 0..input.len() {
		let cur = input.chars().nth(i).unwrap();

		match cur {
			//Quote start/end
			'"' => {
				if prev != '\\' {
					in_quote = !in_quote;
				}
			}
			
			//Object start
			'{' => {
				if !in_quote {
					obj_level += 1;
				}
			}
			
			//Object end
			'}' => {
				if !in_quote {
					obj_level -= 1;
				}
			}

			//Array start
			'[' => {
				if !in_quote {
					arr_level += 1;
				}
			}

			//Array end
			']' => {
				if !in_quote {
					arr_level -= 1;
				}
			}

			//New split index for the current object
			',' => {
				if (obj_level + arr_level == 0) && !in_quote {
					split_indexes.push(i);
				}
			}

			_ => {}
		}

		prev = cur;
	}

	//We know that if these conditions are not met, the JSON object is invalid
	if obj_level != 0 || arr_level != 0 || in_quote {
		println!("JSON: Split was unable to properly parse value.");
		return None;
	}

	return Some(split_indexes);
}

//Parse a JSON object from a string
#[allow(unused)]
pub fn parse_object(input: &str) -> Value {
	let mut to_return = Value::obj();

	//We know if the first character isn't an opening bracket, our json object is invalid
	if input.chars().nth(0).unwrap() != '{' {
		println!("Object does not start with {{");
		return Value::Invalid;
	}

	//Remove whitespace and {} from our object to make parsing easier.
	let trimmed = remove_whitespace(input);
	let trimmed_len = trimmed.len();
	let trimmed = &trimmed[1..trimmed_len-1];

	//Get list of split indexes
	let split_indexes = get_splits(trimmed);
	if split_indexes.is_none() {
		return Value::Invalid;
	}
	let split_indexes = split_indexes.unwrap();

	//Split along all key:value pairs
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

	//Remove ,
	for i in 1..keypairs.len() {
		keypairs[i] = &keypairs[i][1..];
	}

	//Parse all keypairs
	for pair in keypairs {
		let result = parse_keypair(pair);

		//Some part of the data is invalid
		if let Value::Invalid = result.1 {
			return Value::Invalid;
		}

		to_return[result.0] = result.1;
	}

	return to_return;
}


//Parse a string into an array
#[allow(unused)]
pub fn parse_array(input: &str) -> Value{
	let mut to_return = Value::arr();
	
	//We know if the first character of our array isn't [, it is invalid.
	if input.chars().nth(0).unwrap() != '[' {
		println!("JSON: first character of array is not [.");
		return Value::Invalid;
	}

	//Trim whitespace and [] from input
	let trimmed = remove_whitespace(input);
	let trimmed_len = trimmed.len();
	let trimmed = &trimmed[1..trimmed_len-1];

	//Get split indexes
	let split_indexes = get_splits(trimmed);
	if split_indexes.is_none() {
		return Value::Invalid;
	}
	let split_indexes = split_indexes.unwrap();

	//Split into value strings
	let mut prev_index = 0;
	let mut values: Vec<&str> = Vec::new();
	for i in split_indexes {
		let temp = trimmed.split_at(i);
		values.push(&temp.0);
		prev_index = i;
	}
	values.push(trimmed.split_at(prev_index).1);

	//Remove ,
	for i in 1..values.len() {
		values[i] = &values[i][1..];
	}

	//Parse values
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