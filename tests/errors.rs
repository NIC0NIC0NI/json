extern crate json;

use json::JSON;
use std::str::FromStr;

#[test]
fn it_checks_simple_syntax_error() {
    let json_str = stringify!(
        {
            "name" : "Element",
            "items"
            [
                1, 2, 3, false, true, 
                {
                    "something" : null
                }
            ]
        }
    );
    match JSON::from_str(json_str){
        Ok(_) => panic!("Unable to find out syntax errors."),
        Err(_) => ()
    }
}

#[test]
fn it_checks_miss_typing() {
    let json_str = stringify!(
        {
            "name" : "Element",
            "items" :
            [
                1, 2, 3, false, ture, 
                {
                    "something" : null
                }
            ]
        }
    );
    match JSON::from_str(json_str){
        Ok(_) => panic!("Unable to find out syntax errors."),
        Err(_) => ()
    }
}

#[test]
fn it_checks_unbalanced_quote() {
    // unbalanced braces won't compile if using stringfy!
    let json_str1 = "{\"name\" : \"Element,\"items\" :[1, 2, 3, false, ture, {\"something\" : null , false]}";
    match JSON::from_str(json_str1){
        Ok(_) => panic!("Unable to find out syntax errors."),
        Err(_) => ()
    };
    let json_str2 = "{\"name\" : \"Element\", items\" :[1, 2, 3, false, ture, {\"something\" : null , false]}";
    match JSON::from_str(json_str2){
        Ok(_) => panic!("Unable to find out syntax errors."),
        Err(_) => ()
    }
}

#[test]
fn it_checks_unbalanced_brace() {
    // unbalanced braces won't compile if using stringfy!
    let json_str1 = "{\"name\" : \"Element\",\"items\" :[1, 2, 3, false, ture, {\"something\" : null , false]}";
    match JSON::from_str(json_str1){
        Ok(_) => panic!("Unable to find out syntax errors."),
        Err(_) => ()
    };
    let json_str1 = "\"name\" : \"Element\",\"items\" :[1, 2, 3, false, ture, {\"something\" : null} , false]}";
    match JSON::from_str(json_str1){
        Ok(_) => panic!("Unable to find out syntax errors."),
        Err(_) => ()
    }
}

#[test]
fn it_checks_unbalanced_bracket() {
    // unbalanced brackets won't compile if using stringfy!
    let json_str = "{\"name\" : \"Element\",\"items\" :[1, 2, 3, false, ture, {\"something\" : null}}";
    match JSON::from_str(json_str){
        Ok(_) => panic!("Unable to find out syntax errors."),
        Err(_) => ()
    };
    let json_str = "{\"name\" : \"Element\",\"items\" :[1, 2, false, {\"something\" : null}], \"something\": 12]}";
    match JSON::from_str(json_str){
        Ok(_) => panic!("Unable to find out syntax errors."),
        Err(_) => ()
    }
}
