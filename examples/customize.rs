#[macro_use]
extern crate json;

use std::str::FromStr;
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
    match json_str.parse::<MyObject>() {
        Ok(object) => println!("{:?}", object),
        Err(e) => panic!("{}", e)
    }
}

impl FromStr for MyObject {
    type Err = <JSONMaker as FromJSONStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match JSONMaker::from_json_str(s) {
            Ok(JSONMaker::Object(ObjectMaker(object))) => Ok(object),
            Ok(_) => Err("Unexpected structure".into()),
            Err(e) => Err(e)
        }
    }
}

struct NumberMaker(i32);
struct ArrayMaker(Vec<i32>);
struct ObjectMaker(MyObject);
enum JSONMaker {
    Null,Bool,
    String(String), 
    Number(NumberMaker), 
    Array(ArrayMaker), 
    Object(ObjectMaker),
}

impl FromStr for NumberMaker {
    type Err = <i32 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        Ok(NumberMaker(try!(s.parse::<i32>())))
    }
}

impl JSONObject for ObjectMaker {
    type JSON = JSONMaker;
    fn new() -> Self{
        ObjectMaker(MyObject::default())
    }
    fn add(&mut self, name: String, value: Self::JSON){
        let name_str: &str = &name;
        match name_str {
            "name" => {
                if let JSONMaker::String(s) = value {
                    self.0.name = s;
                }
            },
            "numbers" => {
                if let JSONMaker::Array(ArrayMaker(array)) = value {
                    self.0.numbers = array;
                }
            }
            _ => {}
        }
    }
}

impl JSONArray for ArrayMaker {
    type JSON = JSONMaker;
    fn new() -> Self {
        ArrayMaker(Vec::new())
    }
    fn add(&mut self, value: Self::JSON) {
        if let JSONMaker::Number(NumberMaker(i)) = value {
            self.0.push(i);
        }
    }
}

impl MakeJSON for JSONMaker {
    type Array = ArrayMaker;
    type Object = ObjectMaker;
    type Number = NumberMaker;
    fn make_number(n: Self::Number) -> Self {
        JSONMaker::Number(n)
    }
    fn make_null() -> Self {
        JSONMaker::Null
    }
    fn make_string(s: String) -> Self {
        JSONMaker::String(s)
    }
    fn make_bool(_: bool) -> Self {
        JSONMaker::Bool
    }
    fn make_array(array: Self::Array) -> Self {
        JSONMaker::Array(array)
    }
    fn make_object(object: Self::Object) -> Self {
        JSONMaker::Object(object)
    }
}
