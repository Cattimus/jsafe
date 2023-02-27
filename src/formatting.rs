#[path = "json.rs"] mod json;

fn indent(input: &mut String, tab_width: usize, indent_level: usize, spaces: bool) {
	for _ in 0..indent_level {
		if spaces {
			for _ in 0..tab_width {
				input.push(' ');
			}
		} else {
			input.push('\t');
		}
	}
}

//assumes spaces have been removed
pub fn format(to_print: json::Value, tab_width: usize, spaces: bool) -> String {
	let mut to_return = String::from("");

	//convert value to string
	let temp = to_print.to_string();


	let mut in_quote = false;
	let mut prev = ' ';
	let mut indent_level = 0;
	for c in temp.chars() {
		match c {
			'"' => {
				if prev != '\\' {
					in_quote = !in_quote;
				}

				to_return.push(c);
			}

			'{' => {
				if !in_quote {
					indent_level += 1;
					to_return.push(c);
					to_return.push('\n');

					indent(&mut to_return, tab_width, indent_level, spaces);
				}
			}

			'}' => {
				if !in_quote {
					indent_level -= 1;
					to_return.push('\n');
					indent(&mut to_return, tab_width, indent_level, spaces);
					to_return.push(c);
					
				}
			}

			'[' => {
				if !in_quote {
					indent_level += 1;
					to_return.push(c);
					to_return.push('\n');

					indent(&mut to_return, tab_width, indent_level, spaces);
				}
			}

			']' => {
				if !in_quote {
					indent_level -= 1;
					to_return.push('\n');
					indent(&mut to_return, tab_width, indent_level, spaces);
					to_return.push(c);
					
				}
			}

			',' => {
				if !in_quote {
					to_return.push('\n');
					indent(&mut to_return, tab_width, indent_level, spaces);
				}
			}

			_ => {to_return.push(c);}
		}

		prev = c;
	}

	return to_return;
}