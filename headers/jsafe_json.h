#ifndef JSAFE_JSON_H
#define JSAFE_JSON_H

#include "jsafe_types.h"

//Deallocation functions
extern void jsafe_free_value(jsafe_value* val);
extern void jsafe_free_string(const char* str);

//Allocation functions
extern jsafe_value* jsafe_new_obj();
extern jsafe_value* jsafe_new_arr();
extern jsafe_value* jsafe_new_text(const char* text);
extern jsafe_value* jsafe_new_null();
extern jsafe_value* jsafe_new_bool(int val);
extern jsafe_value* jsafe_new_num(double val);

//Set properties
extern void jsafe_add(jsafe_value* root, jsafe_value* to_add); //Append to array
extern void jsafe_get_property(jsafe_value* root, const char* key); //Get/set for object
extern void jsafe_prealloc(jsafe_value* root, unsigned int size);

//Get properties
extern unsigned int jsafe_get_len(jsafe_value* val);
extern const char* jsafe_get_text(jsafe_value* val);
extern double jsafe_get_num(jsafe_value* val);
extern int jsafe_get_bool(jsafe_value* val);
extern const char* jsafe_to_string(jsafe_value* val);

//Check properties
extern int jsafe_has_key(jsafe_value* val, const char* key);
extern int jsafe_is_null(jsafe_value* val);
extern int jsafe_is_valid(jsafe_value* val);
extern int jsafe_is_text(jsafe_value* val);
extern int jsafe_is_num(jsafe_value* val);
extern int jsafe_is_obj(jsafe_value* val);
extern int jsafe_is_arr(jsafe_value* val);

#endif