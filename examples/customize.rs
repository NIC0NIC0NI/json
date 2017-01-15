#[macro_use]
extern crate json;

use std::str::FromStr;
use json::{MakeJSON, JSONObject, JSONArray, FromJSONStr, NestedLevel};

#[derive(Debug, Default)]
struct Attribute {
    name: String,
    value: String
}

#[derive(Debug, Default)]
struct MyObject {
    name: String,
    numbers: Vec<i32>,
    attributes: Vec<Attribute>
}

fn main(){
    let json_str = stringify!(
        {
            "name" : "Fibonacci",
            "numbers" : [1, 1, 2, 3, 5, 8, 13, 21],
            "attributes" : [
                {
                    "name" : "type",
                    "value" : "native structure"
                },
                {
                    "name" : "serial ID",
                    "value" : "0x1254FA78"
                }
            ]
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
            Ok(JSONMaker::Object(ObjectMaker::MyObject(object))) => Ok(object),
            Ok(obj) => Err(format!("Unexpected structure {:?}", obj).into()),
            Err(e) => Err(e)
        }
    }
}

#[derive(Debug)]
struct NumberMaker(i32);
#[derive(Debug)]
enum ArrayMaker {
    Numbers(Vec<i32>),
    Attributes(Vec<Attribute>)
}
#[derive(Debug)]
enum ObjectMaker{
    MyObject(MyObject),
    Attribute(Attribute)
}
#[derive(Debug)]
enum JSONMaker {
    Null,Bool,
    String(String), 
    Number(NumberMaker), 
    Array(ArrayMaker), 
    Object(ObjectMaker)
}

impl FromStr for NumberMaker {
    type Err = <i32 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        Ok(NumberMaker(try!(s.parse::<i32>())))
    }
}

impl JSONObject for ObjectMaker {
    type JSON = JSONMaker;
    fn new(nested:&Vec<NestedLevel<JSONMaker>>) -> Self{
        match nested.len() {
            0 => ObjectMaker::MyObject(MyObject::default()),
            _ => ObjectMaker::Attribute(Attribute::default())
        }
    }
    fn add(&mut self, name: String, value: Self::JSON, nested:&Vec<NestedLevel<JSONMaker>>){
        let name_str: &str = &name;
        match nested.len() {
            0 => {
                match name_str {
                    "name" => {
                        if let JSONMaker::String(s) = value {
                            if let ObjectMaker::MyObject(ref mut obj) = *self {
                                obj.name = s;
                            }
                        }
                    },
                    "numbers" => {
                        if let JSONMaker::Array(ArrayMaker::Numbers(array)) = value {
                            if let ObjectMaker::MyObject(ref mut obj) = *self {
                                obj.numbers = array;
                            }
                        }
                    },
                    "attributes" => {
                        if let JSONMaker::Array(ArrayMaker::Attributes(array)) = value {
                            if let ObjectMaker::MyObject(ref mut obj) = *self {
                                obj.attributes = array;
                            }
                        }
                    },
                    _ => {}
                }
            },
            _ => {
                match name_str {
                    "name" => {
                        if let JSONMaker::String(s) = value {
                            if let ObjectMaker::Attribute(ref mut obj) = *self {
                                obj.name = s;
                            }
                        }
                    },
                    "value" => {
                        if let JSONMaker::String(s) = value {
                            if let ObjectMaker::Attribute(ref mut obj) = *self {
                                obj.value = s;
                            }
                        }
                    },
                    _ => {}
                }
            },
            //_ => {}
        }
    }
}

impl JSONArray for ArrayMaker {
    type JSON = JSONMaker;
    fn new(nested:&Vec<NestedLevel<JSONMaker>>) -> Self {
        match nested.first() {
            Some(&NestedLevel::Object(_, ref name)) if (name == "numbers") 
                => ArrayMaker::Numbers(Vec::new()),
            _ => ArrayMaker::Attributes(Vec::new())
        }
    }
    fn add(&mut self, value: Self::JSON, nested:&Vec<NestedLevel<JSONMaker>>) {
        match nested.first() {
            Some(&NestedLevel::Object(_, ref name)) if (name == "numbers") 
                => {
                if let ArrayMaker::Numbers(ref mut arr) = *self {
                    if let JSONMaker::Number(NumberMaker(i)) = value {
                        arr.push(i);
                    }
                }
            },
            _ => {
                if let ArrayMaker::Attributes(ref mut arr) = *self {
                    if let JSONMaker::Object(ObjectMaker::Attribute(a)) = value {
                        arr.push(a);
                    }
                }
            }
            //_ => {}
        }
        
    }
}

impl MakeJSON for JSONMaker {
    type Array = ArrayMaker;
    type Object = ObjectMaker;
    type Number = NumberMaker;
    fn make_number(n: Self::Number, _:&Vec<NestedLevel<JSONMaker>>) -> Self {
        JSONMaker::Number(n)
    }
    fn make_null(_:&Vec<NestedLevel<JSONMaker>>) -> Self {
        JSONMaker::Null
    }
    fn make_string(s: String, _:&Vec<NestedLevel<JSONMaker>>) -> Self {
        JSONMaker::String(s)
    }
    fn make_bool(_: bool, _:&Vec<NestedLevel<JSONMaker>>) -> Self {
        JSONMaker::Bool
    }
    fn make_array(array: Self::Array, _:&Vec<NestedLevel<JSONMaker>>) -> Self {
        JSONMaker::Array(array)
    }
    fn make_object(object: Self::Object, _:&Vec<NestedLevel<JSONMaker>>) -> Self {
        JSONMaker::Object(object)
    }
}
