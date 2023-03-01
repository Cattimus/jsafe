use json::Value;
use json::parsing;

use std::ffi::{CStr, c_char};

//Get a value from a string
#[no_mangle]
pub unsafe extern "C" fn jsafe_from_str(text: *const c_char) -> Box<Value>{
	let str = CStr::from_ptr(text).to_str();
	if str.is_err() {
		return Box::new(Value::Invalid);
	}

	let str = str.unwrap();
	Box::new(parsing::from_str(str))
}