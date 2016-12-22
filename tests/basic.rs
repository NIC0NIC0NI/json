#[macro_use]
extern crate json;

use json::DefaultJSON as JSON;

#[test]
fn it_seems_to_work() {
    let json_obj = json_default!(
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
    match json_str.parse::<JSON>(){
        Ok(parsed) => assert_eq!(parsed, json_obj),
        Err(msg) => panic!("{}:\n{}", msg, json_str)
    }
}

#[test]
fn it_works() {
    let json_obj = json_default!(
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
    match json_str.parse::<JSON>(){
        Ok(parsed) => assert_eq!(parsed, json_obj),
        Err(msg) => panic!("{}:\n{}", msg, json_str)
    }
}


#[test]
fn it_works_with_premitives() {
    let a = stringify!("a single string");
    let b = stringify!(12345);
    let c = stringify!(123.45);
    let d = stringify!(null);
    let e = stringify!(false);
    match a.parse::<JSON>() {
        Ok(parsed) => assert_eq!(parsed, json_default!("a single string")),
        Err(msg) => panic!("{}:\n{}", msg, a)
    }
    match b.parse::<JSON>() {
        Ok(parsed) => assert_eq!(parsed, json_default!(12345)),
        Err(msg) => panic!("{}:\n{}", msg, b)
    }
    match c.parse::<JSON>() {
        Ok(parsed) => assert_eq!(parsed, json_default!(123.45)),
        Err(msg) => panic!("{}:\n{}", msg, c)
    }
    match d.parse::<JSON>() {
        Ok(parsed) => assert_eq!(parsed, json_default!(null)),
        Err(msg) => panic!("{}:\n{}", msg, d)
    }
    match e.parse::<JSON>() {
        Ok(parsed) => assert_eq!(parsed, json_default!(false)),
        Err(msg) => panic!("{}:\n{}", msg, e)
    }
}
