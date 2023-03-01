use json::Value;
use json::formatting;

use std::ffi::{CString};

#[no_mangle]
pub unsafe extern "C" fn jsafe_to_pretty(to_print: *mut Value, spaces: usize) -> Box<CString> {
	if to_print.is_null() {
		return Box::new(CString::new("").unwrap());
	}

	return Box::new(CString::new(formatting::format(to_print.as_ref().unwrap(), spaces).as_bytes()).unwrap());
}