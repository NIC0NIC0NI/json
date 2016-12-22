extern crate json;

use json::DefaultJSON as JSON;

#[test]
fn it_checks_simple_syntax_error_1() {
    let json_str = stringify!(
        {
            "name" : "Element",
            "items" :
            [
                1, 2, 3, false  true, 
                {
                    "something" : null
                }
            ]
        }
    );
    assert!(json_str.parse::<JSON>().is_err());
}

#[test]
fn it_checks_simple_syntax_error_2() {
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
    assert!(json_str.parse::<JSON>().is_err());
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
    assert!(json_str.parse::<JSON>().is_err());
}

#[test]
fn it_checks_unbalanced_quote() {
    // unbalanced quotes won't compile if using stringfy!
    let json_str1 = "{\"name\" : \"Element,\"items\" :[1, 2, 3, false, ture, {\"something\" : null , false]}";
    let json_str2 = "{\"name\" : \"Element\", items\" :[1, 2, 3, false, ture, {\"something\" : null , false]}";

    assert!(json_str1.parse::<JSON>().is_err());
    assert!(json_str2.parse::<JSON>().is_err());
}

#[test]
fn it_checks_unbalanced_brace() {
    // unbalanced braces won't compile if using stringfy!
    let json_str1 = "{\"name\" : \"Element\",\"items\" :[1, 2, 3, false, ture, {\"something\" : null , false]}";
    let json_str2 = "\"name\" : \"Element\",\"items\" :[1, 2, 3, false, ture, {\"something\" : null} , false]}";

    assert!(json_str1.parse::<JSON>().is_err());
    assert!(json_str2.parse::<JSON>().is_err());
}

#[test]
fn it_checks_unbalanced_bracket() {
    // unbalanced brackets won't compile if using stringfy!
    let json_str1 = "{\"name\" : \"Element\",\"items\" :[1, 2, 3, false, ture, {\"something\" : null}}";
    let json_str2 = "{\"name\" : \"Element\",\"items\" :[1, 2, false, {\"something\" : null}], \"something\": 12]}";

    assert!(json_str1.parse::<JSON>().is_err());
    assert!(json_str2.parse::<JSON>().is_err());
}

#[test]
fn it_checks_empty() {
    let empty = "";
    assert!(empty.parse::<JSON>().is_err());
}

