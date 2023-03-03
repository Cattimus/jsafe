use json::formatting;

use std::ffi::{CString};

use super::c_json::{object,create_string,object_is_null};

#[no_mangle]
pub unsafe extern "C" fn jsafe_to_pretty(this: *mut object, spaces: usize) -> *mut CString {
	if object_is_null(this) {
		return create_string(CString::new("").unwrap());
	}

	return create_string(CString::new(formatting::prettify((*(*this).current).as_ref(), spaces).as_bytes()).unwrap());
}