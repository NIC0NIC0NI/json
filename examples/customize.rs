#[macro_use]
extern crate json;

use json::{MakeJSON, JSONObject, JSONArray, FromJSONStr};
use std::str::FromStr;

#[derive(Debug, Default)]
struct MyObject {
    name: String,
    numbers: Vec<i32>
}

fn main(){
    let json_str = stringify!(
        {
            "name" : "Fibonacci",
            "numbers" : [1, 1, 2, 3, 5, 8, 13, 21]
        }
    );
    let object_maker = MyObjectMaker::from_json_str(json_str);
    match object_maker {
        Ok(MyObjectMaker::MyObject(object)) => {
            println!("{:?}", object)
        },
        _ => panic!("Error")
    }
}

// implement JSONArray for Vec<i32> won't compile
// because both JSONArray and Vec<i32> are not inside the crate
struct ArrayWrapper(Vec<i32>); 

enum MyObjectMaker {
    Int(i32), 
    String(String), 
    IntArray(ArrayWrapper), 
    MyObject(MyObject),
    Err
}

impl JSONObject for MyObject {
    type JSON = MyObjectMaker;
    fn new() -> Self{
        MyObject::default()
    }
    fn add(&mut self, name: String, value: Self::JSON){
        if name == "name" {
            if let MyObjectMaker::String(s) = value {
                self.name = s;
            }
        } else if name == "numbers" {
            if let MyObjectMaker::IntArray(array) = value {
                self.numbers = array.0;
            }
        }
    }
}

impl JSONArray for ArrayWrapper {
    type JSON = MyObjectMaker;
    fn new() -> Self {
        ArrayWrapper(Vec::new())
    }
    fn add(&mut self, value: Self::JSON) {
        if let MyObjectMaker::Int(i) = value {
            self.0.push(i);
        }
    }
}

impl MakeJSON for MyObjectMaker {
    type Array = ArrayWrapper;
    type Object = MyObject;
    
    // Number types are chosen by user.
    // They handle overflows.
    // Currently they must also handle syntax error on number literals
    // Which may be fixed in later versions
    fn make_number(s: &str) -> Option<Self> {
        if let Ok(i) = i32::from_str(s) {
            Some(MyObjectMaker::Int(i))
        } else {
            None
        }
    }
    fn make_null() -> Self {
        MyObjectMaker::Err
    }
    fn make_string(s: String) -> Self {
        MyObjectMaker::String(s)
    }
    fn make_bool(_: bool) -> Self {
        MyObjectMaker::Err
    }
    fn make_array(array: Self::Array) -> Self {
        MyObjectMaker::IntArray(array)
    }
    fn make_object(object: Self::Object) -> Self {
        MyObjectMaker::MyObject(object)
    }
}
