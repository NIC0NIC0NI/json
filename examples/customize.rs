#[macro_use]
extern crate json;

use json::{MakeJSON, JSONObject, JSONArray, FromJSONStr};

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

// 

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
        let name_str: &str = &name;
        match name_str {
            "name" => {
                if let MyObjectMaker::String(s) = value {
                    self.name = s;
                }
            },
            "numbers" => {
                if let MyObjectMaker::IntArray(array) = value {
                    self.numbers = array.0;
                }
            }
            _ => {}
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
    type Number = i32;
    fn make_number(i: i32) -> Self {
        MyObjectMaker::Int(i)
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
