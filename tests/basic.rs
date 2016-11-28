#[macro_use(json_object)]
extern crate json;

use json::JSON;
use std::str::FromStr;

#[test]
fn it_works() {
    let json_obj = json_object!(
        {
            class : "Element",
            tag : "a",
            attributes :
            [
                {
                    class : "Attribute",
                    key : "href",
                    value : "/publish/newthu/newthu_cnt/faculties/index.html"
                }
            ],
            children :
            [
                {
                    class : "TextNode",
                    text : "JSON",
                    attributes : [],
                    children : []
                }
            ]
        }
    );
    let json_str = stringify!(
        {
            "class" : "Element",
            "tag" : "a",
            "attributes" :
            [
                {
                    "class" : "Attribute",
                    "key" : "href",
                    "value" : "/publish/newthu/newthu_cnt/faculties/index.html"
                }
            ],
            "children" :
            [
                {
                    "class" : "TextNode",
                    "text" : "JSON",
                    "attributes" : [],
                    "children" : []
                }
            ]
        }
    );
    match JSON::from_str(json_str){
        Ok(parsed) => assert_eq!(parsed, json_obj),
        Err(msg) => panic!("{}:\n{}", msg, json_str)
    }
}

#[test]
fn it_seems_to_work() {
    let json_obj = json_object!(
        {
            name : "Element",
            items : 
            [
                1, 2, 3, false, true, 
                {
                    something : null
                }
            ]
        }
    );
    let json_str = stringify!(
        {
            "name" : "Element",
            "items" : 
            [
                1, 2, 3, false, true, 
                {
                    "something" : null
                }
            ]
        }
    );
    match JSON::from_str(json_str){
        Ok(parsed) => assert_eq!(parsed, json_obj),
        Err(msg) => panic!("{}:\n{}", msg, json_str)
    }
}
