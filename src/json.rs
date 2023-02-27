use std::collections::HashMap;
use std::ops::Index;
use std::ops::IndexMut;

pub enum Value {
	Invalid,
	Null,
	Number(f64),
	Text(String),
	Bool(bool),
	Object(HashMap<String, Value>),
	Array(Vec<Value>)
}

impl From<f64> for Value {
	fn from(input: f64) -> Self {
		return Value::Number(input);
	}
}
impl From<i64> for Value {
	fn from(input: i64) -> Self {
		return Value::Number(input as f64);
	}
}
impl Into<f64> for &Value {
	fn into(self) -> f64{
		match self {
			Value::Number(x) => *x,
			_ => {
				println!("WARNING: value is not a number and has been converted to 0 by default");
				0.0
			}
		}
	}
}
impl Into<i64> for &Value {
	fn into(self) -> i64{
		match self {
			Value::Number(x) => *x as i64,
			_ => {
				println!("WARNING: value is not a number and has been converted to 0 by default");
				0
			}
		}
	}
}

impl From<String> for Value {
	fn from(input: String) -> Self {
		return Value::Text(input);
	}
}
impl Into<String> for &Value {
	fn into(self) -> String {
		match self {
			Value::Text(x) => String::clone(x),
			_ => {
				println!("WARNING: value is not a string and has been converted to empty str by default");
				String::from("")
			}
		}
	}
}
impl From<&str> for Value {
	fn from(input: &str) -> Self {
		return Value::Text(input.to_string());
	}
}

impl From<bool> for Value {
	fn from(input: bool) -> Self {
		return Value::Bool(input);
	}
}
impl Into<bool> for &Value {
	fn into(self) -> bool {
		match self {
			Value::Bool(x) => *x,
			_ => {
				println!("WARNING: value is not a bool and has been converted to false by default");
				false
			}
		}
	}
}

impl From<HashMap<String, Value>> for Value {
	fn from(input: HashMap<String, Value>) -> Self {
		return Value::Object(input);
	}
}

impl From<Vec<Value>> for Value {
	fn from(input: Vec<Value>) -> Self {
		return Value::Array(input);
	}
}

impl Value {
	pub fn obj() -> Value {
		return Value::Object(HashMap::new());
	}
	pub fn from_str(_input: &str) -> Value {
		return Value::Null;
	}

	pub fn arr() -> Value {
		return Value::Array(Vec::new());
	}

	pub fn len(&self) -> usize {
		match self {
    		Value::Array(x) => x.len(),
			Value::Object(x) => x.len(),
    		_ => 0
		}
	}

	pub fn has(&self, key: &str) -> bool {
		match self {
			Value::Object(x) => x.contains_key(key),
			_ => false
		}
	}

	pub fn append(&mut self, val: Value) {
		match self {
			Value::Array(x) => x.push(val),
			_ => println!("WARNING: attempted to append to a value that is not an array. Nothing will be done.")
		}
	}

	pub fn to_string(&self) -> String {
		let mut to_return = String::from("");

		match self {
			Value::Invalid => to_return += "INVALID",
			Value::Null => to_return += "null",
			Value::Number(x) => to_return += &x.to_string(),

			Value::Text(x) => {
				to_return += "\"";
				to_return += x;
				to_return += "\"";
			},

			Value::Bool(x) => if *x {to_return += "true"} else {to_return += "false"},

			Value::Object(x) => {
				to_return += "{";

				for value in x {
					to_return += "\"";
					to_return += value.0;
					to_return += "\"";

					to_return += ":";
					to_return += &Value::to_string(value.1);
					to_return += ","
				}

				//pop the last comma off the end
				if x.len() > 0 {
					to_return.pop();
				}
				
				to_return += "}";
			},

			Value::Array(x) => {
				to_return += "[";
				for value in x {
					to_return += &Value::to_string(value);
					to_return += ",";
				}

				if x.len() > 0 {
					to_return.pop();
				}
				
				to_return += "]";
			}
		}

		return to_return;
	}

	pub fn as_ref(&self) -> &Value {
		return self;
	}
	pub fn as_mut(&mut self) -> &mut Value {
		return self;
	}

}

impl Index<usize> for Value {
	type Output = Value;

	fn index(&self, index: usize) -> &Self::Output {
		if let Value::Array(x) = self {
			if x.len() > index {
				return &x[index];			
			}
		}

		//value is not an array
		return &Value::Invalid;
	}
}

impl IndexMut<usize> for Value {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		if let Value::Array(x) = self {
			if x.len() > index {
				return &mut x[index];
			}

			//we need a valid mutable index to return, so we create a null one
			println!("JSON: mutable reference has been created to an index that doesn't exist.");
			x.push(Value::Null);
			return x.last_mut().unwrap();
		}

		//value is not an array(we still need to return a valid mutable reference)
		println!("JSON: mutable reference from Value that is not an array. Object has been overwritten");
		return self;
	}
}

impl Index<&str> for Value {
	type Output = Value;

	fn index(&self, index: &str) -> &Self::Output {
		if let Value::Object(x) = self {
			if x.contains_key(index) {
				return &x[index];
			}
		}

		//value is not an object
		return &Value::Invalid;
	}
}

impl IndexMut<&str> for Value {
	fn index_mut(&mut self, index: &str) -> &mut Self::Output {
		if let Value::Object(x) = self {
			if x.contains_key(index) {
				return x.get_mut(index).unwrap();
			}

			//We need to return a mutable reference still
			x.insert(index.to_string(), Value::Null);
			return x.get_mut(index).unwrap();
		}

		//value is not an object (we still have to return a valid mutable reference)
		println!("JSON: mutable reference from Value that is not an Object. Object has been overwritten");
		return self;
	}
}