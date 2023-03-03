use json::Value;
use std::ffi::{CStr, CString, c_char, c_int, c_double, c_uint};
use std::ptr::{null_mut};
use std::sync::Mutex;

//keep track of objects allocated from our library
static OBJECTS: Mutex<Vec<Box<object>>> = Mutex::new(Vec::new());
static STRINGS: Mutex<Vec<Box<CString>>> = Mutex::new(Vec::new());

//We do not want objects to spawn pointers that will become invalid when the object is deleted
pub struct object {
	root: Value,
	current: *mut Value
}

//tell rust that we are only referencing values in our struct
unsafe impl Send for object {}

//Helper function to add a new object to the global pool
fn create_object(to_add: Value) -> *mut object {
	let mut val = Box::new(object {
		root: to_add,
		current: null_mut()
	});

	//initialize current
	val.current = &mut val.root;

	//get a pointer to our object
	let to_return: *mut object = val.as_mut();

	//append object to our array
	OBJECTS.lock().unwrap().push(val);

	//send our object to C
	return to_return;
}

//Helper function to initialize a new string to the global pool
fn create_string(to_add: CString) -> *mut CString {
	let mut val = Box::new(to_add);

	let to_return: *mut CString = val.as_mut();

	STRINGS.lock().unwrap().push(val);

	return to_return;
}

//Helper function to check if an object is null
unsafe fn object_is_null(this: *mut object) -> bool{
	this.is_null() || (*this).current.is_null()
}

//Free a value
#[no_mangle]
pub unsafe extern "C" fn jsafe_free_value(this: *mut object) {
	let objects = &mut OBJECTS.lock().unwrap();

	for i in 0..objects.len() {
		let cur: *mut object = objects[i].as_mut();

		//we are referencing the same object
		if std::ptr::eq(this, cur) {

			//delete the object from the global reference pool
			objects.remove(i);
			break;
		}
	}
}

//Free a string
#[no_mangle]
pub extern "C" fn jsafe_free_string(this: *mut CString) {
	let strings = &mut STRINGS.lock().unwrap();

	for i in 0..strings.len() {
		let cur: *mut CString = strings[i].as_mut();

		//we are referencing the same object
		if std::ptr::eq(this, cur) {

			//delete the object from the global reference pool
			strings.remove(i);
			break;
		}
	}
}

//Free all memory
#[no_mangle]
pub extern "C" fn jsafe_cleanup() {
	let mut objects = OBJECTS.lock().unwrap();
	objects.clear();
	objects.shrink_to(0);

	let mut strings = STRINGS.lock().unwrap();
	strings.clear();
	strings.shrink_to(0);
}

//Return a new globally allocated object
#[no_mangle]
pub extern "C" fn jsafe_new_root() -> *mut object {
	create_object(Value::obj())
}

//Return a reference to a new json object
#[no_mangle]
pub extern "C" fn jsafe_new_obj() -> Box<Value> {
	Box::new(Value::obj())
}

//Return a reference to a new json array
#[no_mangle]
pub extern "C" fn jsafe_new_arr() -> Box<Value> {
	Box::new(Value::arr())
}

//Return a new text object from a C string
#[no_mangle]
pub unsafe extern "C" fn jsafe_new_text(text: *const c_char) -> Box<Value> {

	if text.is_null() {
		return Box::new(Value::Text("".to_string()));
	}

	//convert text to a string
	let str = CStr::from_ptr(text).to_str().unwrap();
	Box::new(Value::Text(str.to_string()))
}

//Return a new null object
#[no_mangle]
pub extern "C" fn jsafe_new_null() -> Box<Value> {
	Box::new(Value::Null)
}

//Return a new bool object
#[no_mangle]
pub extern "C" fn jsafe_new_bool(bool: c_int) -> Box<Value> {
	Box::new(Value::Bool(!(bool == 0)))
}

//Return a new number object
#[no_mangle]
pub extern "C" fn jsafe_new_num(val: c_double) -> Box<Value> {
	Box::new(Value::Number(val))
}

//Add a new value to an array. This will take ownership of the pointer.
#[no_mangle]
pub unsafe extern "C" fn jsafe_add(this: *mut object, to_add: Option<Box<Value>>) {
	if object_is_null(this) {
		return;
	}

	//shorthand to add a null if you run jsafe_add(val, NULL)
	if to_add.is_none() {
		(*(*this).current).add(Value::Null);
	}

	//add item to list
	(*(*this).current).add(*to_add.unwrap());
}

//Pre-allocate slots for the container (to speed up adding values)
#[no_mangle]
pub unsafe extern "C" fn jsafe_prealloc(this: *mut object, amount: usize) {
	if object_is_null(this) {
		return;
	}

	(*(*this).current).pre_alloc(amount);
}

//Get a pointer to a Value from a string index
#[no_mangle]
pub unsafe extern "C" fn jsafe_get_property(this: *mut object, key: *const c_char) {
	if  object_is_null(this) {
		return;
	}

	let str = CStr::from_ptr(key).to_str().unwrap();

	//replace current with new value
	(*this).current = (*(*this).current)[str].as_mut();
}

//Set a value from string index. This will free the value passed to it
#[no_mangle]
pub unsafe extern "C" fn jsafe_set_property(this: *mut object, key: *const c_char, val: Option<Box<Value>>) {
	if object_is_null(this) {
		return;
	}

	//accounting for NULL
	let str = CStr::from_ptr(key).to_str().unwrap();
	if val.is_none() {
		(*(*this).current)[str] = Value::Null;
	} else {
		(*(*this).current)[str] = *val.unwrap();
	}
}

//Get a pointer to a value from a number index
#[no_mangle]
pub unsafe extern "C" fn jsafe_get_index(this: *mut object, key: usize) {
	if object_is_null(this) {
		return;
	}

	if key >= (*(*this).current).len() {
		return;
	}

	(*this).current = (*(*this).current)[key].as_mut();
}

//Return a string representation of an object
#[no_mangle]
pub unsafe extern "C" fn jsafe_to_string(this: *mut object) -> *mut CString {
	if object_is_null(this) {
		return create_string(CString::new("Null").unwrap());
	}

	let str = (*(*this).current).to_string();
	create_string(CString::new(str).unwrap())
}

//Get the length of a json array/object
#[no_mangle]
pub unsafe extern "C" fn jsafe_get_len(this: *mut object) -> c_uint {
	if object_is_null(this) {
		return 0;
	}

	(*(*this).current).len() as u32
}

//Check if a json object has a key
#[no_mangle]
pub unsafe extern "C" fn jsafe_has_key(this: *mut object, key: *const c_char) -> c_int {
	if object_is_null(this) || key.is_null() {
		return 0;
	}

	let str = CStr::from_ptr(key).to_str().unwrap();
	(*(*this).current).has(str) as c_int
}

//Get a string value from an object
#[no_mangle]
pub unsafe extern "C" fn jsafe_get_text(this: *mut object) -> *mut CString {
	if object_is_null(this) {
		return null_mut();
	}

	let val = (*this).current.as_ref().unwrap();
	match val {
		Value::Text(x) => create_string(CString::new(x.as_bytes()).unwrap()),
		_ => null_mut()
	}
}

//Get a number value from an object
#[no_mangle]
pub unsafe extern "C" fn jsafe_get_num(this: *mut object) -> c_double {
	if object_is_null(this) {
		return 0.0;
	}

	let val = (*this).current.as_ref().unwrap();
	match val {
		Value::Number(x) => *x,
		_ => 0.0
	}
}

//Get a number value from an object
#[no_mangle]
pub unsafe extern "C" fn jsafe_get_bool(this: *mut object) -> c_int {
	if object_is_null(this) {
		return 0;
	}

	let val = (*this).current.as_ref().unwrap();
	match val {
		Value::Bool(x) => *x as c_int,
		_ => 0
	}
}

//Check if a value is null
#[no_mangle]
pub unsafe extern "C" fn jsafe_is_null(this: *mut object) -> c_int {
	if object_is_null(this) {
		return 0;
	}

	let val = (*this).current.as_ref().unwrap();
	match val {
		Value::Null => 1,
		_ => 0
	}
}

//Check if a value is invalid
#[no_mangle]
pub unsafe extern "C" fn jsafe_is_valid(this: *mut object) -> c_int {
	if object_is_null(this) {
		return 0;
	}

	let val = (*this).current.as_ref().unwrap();
	match val {
		Value::Invalid => 0,
		_ => 1
	}
}

//Check if a value is text
#[no_mangle]
pub unsafe extern "C" fn jsafe_is_text(this: *mut object) -> c_int {
	if object_is_null(this) {
		return 1;
	}

	let val = (*this).current.as_ref().unwrap();
	match val {
		Value::Text(_) => 1,
		_ => 0
	}
}

//Check if a value is a number
#[no_mangle]
pub unsafe extern "C" fn jsafe_is_num(this: *mut object) -> c_int {
	if object_is_null(this) {
		return 1;
	}

	let val = (*this).current.as_ref().unwrap();
	match val {
		Value::Number(_) => 1,
		_ => 0
	}
}

//Check if a value is an object
#[no_mangle]
pub unsafe extern "C" fn jsafe_is_obj(this: *mut object) -> c_int {
	if object_is_null(this) {
		return 1;
	}

	let val = (*this).current.as_ref().unwrap();
	match val {
		Value::Object(_) => 1,
		_ => 0
	}
}

//Check if a value is an array
#[no_mangle]
pub unsafe extern "C" fn jsafe_is_arr(this: *mut object) -> c_int {
	if object_is_null(this) {
		return 1;
	}

	let val = (*this).current.as_ref().unwrap();
	match val {
		Value::Array(_) => 1,
		_ => 0
	}
}