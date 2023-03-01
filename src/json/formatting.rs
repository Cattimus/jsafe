use crate::json::Value;
#[allow(unused)]

//Helper function to make indenting easier
#[allow(unused)]
fn indent(input: &mut String, indent_level: usize, spaces: bool) {
	for _ in 0..indent_level {
		if spaces {
			input.push(' ');
		} else {
			input.push('\t');
		}
	}
}

//Format a json object from condensed text to human-readable format.
#[allow(unused)]
pub fn format(to_print: &Value, spaces: usize) -> String {
	let mut to_return = String::from("");

	//Check if the object we are trying to format is an object or array
	match to_print {
		Value::Object(_) => (),
		Value::Array(_) => (),

		//If it isn't, return the object converted to a string
		_ => {return to_print.to_string();}
	}

	//Convert value to string
	let obj_str = to_print.to_string();
	//Whether we're in a quote or not
	let mut in_quote = false;
	//Previous character
	let mut prev = ' ';
	//How many tabs/spaces to put before a line
	let mut indent_level = 0;

	//Calculate how big a tab step should be
	let mut tab_width = 1;
	if spaces > 0 {
		tab_width = spaces;
	}

	for c in obj_str.chars() {
		match c {

			//Text block
			'"' => {
				if prev != '\\' {
					in_quote = !in_quote;
				}

				to_return.push(c);
			}

			//Object start
			'{' => {
				if !in_quote {
					indent_level += tab_width;
					to_return.push(c);
					to_return.push('\n');

					indent(&mut to_return, indent_level, spaces > 0);
				}
			}

			//Object end
			'}' => {
				if !in_quote {
					indent_level -= tab_width;
					to_return.push('\n');
					indent(&mut to_return, indent_level, spaces > 0);
					to_return.push(c);
					
				}
			}

			//Array Start
			'[' => {
				if !in_quote {
					indent_level += tab_width;
					to_return.push(c);
					to_return.push('\n');

					indent(&mut to_return, indent_level, spaces > 0);
				}
			}

			//Array end
			']' => {
				if !in_quote {
					indent_level -= tab_width;
					to_return.push('\n');
					indent(&mut to_return, indent_level, spaces > 0);
					to_return.push(c);
					
				}
			}

			//Value separator
			',' => {
				if !in_quote {
					to_return.push(c);
					to_return.push('\n');
					indent(&mut to_return, indent_level, spaces > 0);
				}
			}

			//Generic character
			_ => {to_return.push(c);}
		}

		prev = c;
	}

	return to_return;
}