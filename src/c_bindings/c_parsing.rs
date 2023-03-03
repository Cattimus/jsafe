use json::Value;
use json::parsing;

use std::ffi::{CStr, c_char};
use super::c_json::{object, create_object};

//Get a value from a string
#[no_mangle]
pub unsafe extern "C" fn jsafe_from_str(text: *const c_char) -> *mut object {
	let str = CStr::from_ptr(text).to_str();
	if str.is_err() {
		return create_object(Value::Invalid);
	}

	let str = str.unwrap();
	create_object(parsing::from_str(str))
}