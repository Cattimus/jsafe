#ifndef JSAFE_JSON_H
#define JSAFE_JSON_H

#include "jsafe_types.h"

//Deallocation functions
extern "C" void jsafe_free_value(jsafe_object* val);
extern "C" void jsafe_free_string(jsafe_string* str);
extern "C" void jsafe_cleanup();

//Allocation functions
extern "C" jsafe_object* jsafe_new_root();
extern "C" jsafe_value* jsafe_new_obj();
extern "C" jsafe_value* jsafe_new_arr();
extern "C" jsafe_value* jsafe_new_text(const char* text);
extern "C" jsafe_value* jsafe_new_null();
extern "C" jsafe_value* jsafe_new_bool(int val);
extern "C" jsafe_value* jsafe_new_num(double val);

//Set properties
extern "C" void jsafe_add(jsafe_object* root, jsafe_value* to_add); //Append to array
extern "C" void jsafe_get_property(jsafe_object* root, const char* key); //Get for object
extern "C" void jsafe_set_property(jsafe_object* root, const char* key, jsafe_value* val); //set for object
extern "C" void jsafe_prealloc(jsafe_object* root, unsigned int size);

//Get properties
extern "C" unsigned int jsafe_get_len(jsafe_object* val);
extern "C" jsafe_string* jsafe_get_text(jsafe_object* val);
extern "C" double jsafe_get_num(jsafe_object* val);
extern "C" int jsafe_get_bool(jsafe_object* val);
extern "C" jsafe_string* jsafe_to_string(jsafe_object* val);

//Check properties
extern "C" int jsafe_has_key(jsafe_object* val, const char* key);
extern "C" int jsafe_is_null(jsafe_object* val);
extern "C" int jsafe_is_valid(jsafe_object* val);
extern "C" int jsafe_is_text(jsafe_object* val);
extern "C" int jsafe_is_num(jsafe_object* val);
extern "C" int jsafe_is_obj(jsafe_object* val);
extern "C" int jsafe_is_arr(jsafe_object* val);

#endif