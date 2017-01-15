//! Describes the state transition

use ::std::str::FromStr;
use ::std::error::Error;

use super::super::super::type_adapt::{MakeJSON, JSONObject, JSONArray};
use super::super::{JSONToken, make_parse_error, NestedLevel};
use super::State;

fn token_error<JSON>(token: JSONToken) -> State <JSON> 
    where JSON: MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error {
    State::Error(format!("Unexpected token {}", token).into())
}

fn end_nested<JSON>(mut nested: Vec<NestedLevel<JSON>>, json: JSON) -> State<JSON> 
    where JSON: MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error {
    if let Some(up) = nested.pop() {
        match up {
            NestedLevel::Array(mut array) => {
                array.add(json, &nested);
                State::ArrayWithValue(nested, array)
            },
            NestedLevel::Object(mut object, name) => {
                object.add(name, json, &nested);
                State::ObjectWithValue(nested, object)
            }
        }
    } else {
        State::End(json)
    }
}

pub fn match_begin<JSON>(token:JSONToken) -> State<JSON> 
    where JSON: MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error {
    let nested = Vec::new();
    match token {
        JSONToken::LeftBrace => {
            let obj = <JSON as MakeJSON>::Object::new(&nested);
            State::ObjectBegin(nested, obj)
        },
        JSONToken::LeftBracket => {
            let arr = <JSON as MakeJSON>::Array::new(&nested);
            State::ArrayBegin(nested, arr)
        },
        JSONToken::StringToken(s) => State::End(JSON::make_string(s, &nested)),
        JSONToken::BoolToken(b) => State::End(JSON::make_bool(b, &nested)),
        JSONToken::NumberToken(s) => {
            match s.parse::<<JSON as MakeJSON>::Number>() {
                Ok(n) => State::End(JSON::make_number(n, &nested)),
                Err(e) => State::Error(make_parse_error(e))
            }
        },
        JSONToken::NullToken => State::End(JSON::make_null(&nested)),
        unexpected => token_error(unexpected)
    }
}

pub fn match_object_begin<JSON>(token:JSONToken, 
    nested: Vec<NestedLevel<JSON>>, 
    object: <JSON as MakeJSON>::Object) -> State<JSON>
    where JSON: MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error {
    match token {
        JSONToken::StringToken(name) => State::ObjectWithName(nested, object, name),
        JSONToken::RightBrace => {
            let obj = JSON::make_object(object, &nested);
            end_nested(nested, obj)
        },
        unexpected => token_error(unexpected)
    }
}

pub fn match_object_with_name<JSON>(token:JSONToken, 
    nested: Vec<NestedLevel<JSON>>, 
    object: <JSON as MakeJSON>::Object, name: String) -> State<JSON> 
    where JSON: MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error {
    match token {
        JSONToken::Colon => State::ObjectWithColon(nested, object, name),
        unexpected => token_error(unexpected)
    }
}

pub fn match_object_with_colon<JSON>(token:JSONToken, 
    mut nested: Vec<NestedLevel<JSON>>, 
    mut object: <JSON as MakeJSON>::Object, 
    name: String) -> State<JSON>
    where JSON: MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error {
    match token {
        JSONToken::StringToken(s) => {
            object.add(name, JSON::make_string(s, &nested), &nested);
            State::ObjectWithValue(nested, object)
        },
        JSONToken::BoolToken(b) => {
            object.add(name, JSON::make_bool(b, &nested), &nested);
            State::ObjectWithValue(nested, object)
        },
        JSONToken::NullToken => {
            object.add(name, JSON::make_null(&nested), &nested);
            State::ObjectWithValue(nested, object)
        },
        JSONToken::NumberToken(s) => {
            match s.parse::<<JSON as MakeJSON>::Number>() {
                Ok(num) => {
                    object.add(name, JSON::make_number(num, &nested), &nested);
                    State::ObjectWithValue(nested, object)
                },
                Err(e) => State::Error(make_parse_error(e))
            }
        },
        JSONToken::LeftBrace => {
            nested.push(NestedLevel::Object(object, name));
            let obj = <JSON as MakeJSON>::Object::new(&nested);
            State::ObjectBegin(nested, obj)
        },
        JSONToken::LeftBracket => {
            nested.push(NestedLevel::Object(object, name));
            let arr = <JSON as MakeJSON>::Array::new(&nested);
            State::ArrayBegin(nested, arr)
        },
        unexpected => token_error(unexpected)
    }
    
}

pub fn match_object_with_value<JSON>(token:JSONToken, 
    nested: Vec<NestedLevel<JSON>>, 
    object: <JSON as MakeJSON>::Object) -> State<JSON>
    where JSON: MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error {
    match token {
        JSONToken::Comma => State::ObjectWithComma(nested, object),
        JSONToken::RightBrace => {
            let obj = JSON::make_object(object, &nested);
            end_nested(nested, obj)
        },
        unexpected => token_error(unexpected)
    }
}

pub fn match_object_with_comma<JSON>(token:JSONToken, 
    nested: Vec<NestedLevel<JSON>>, 
    object: <JSON as MakeJSON>::Object) -> State<JSON>
    where JSON: MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error {
    match token {
        JSONToken::StringToken(name) => State::ObjectWithName(nested, object, name),
        unexpected => token_error(unexpected)
    }
}

pub fn match_array_begin<JSON>(token:JSONToken, 
    mut nested: Vec<NestedLevel<JSON>>, 
    mut array: <JSON as MakeJSON>::Array) -> State<JSON>
    where JSON: MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error {
    match token {
        JSONToken::StringToken(s) => {
            array.add(JSON::make_string(s, &nested), &nested);
            State::ArrayWithValue(nested, array)
        },
        JSONToken::BoolToken(b) => {
            array.add(JSON::make_bool(b, &nested), &nested);
            State::ArrayWithValue(nested, array)
        },
        JSONToken::NullToken => {
            array.add(JSON::make_null(&nested), &nested);
            State::ArrayWithValue(nested, array)
        },
        JSONToken::NumberToken(s) => {
            match s.parse::<<JSON as MakeJSON>::Number>() {
                Ok(num) => {
                    array.add(JSON::make_number(num, &nested), &nested);
                    State::ArrayWithValue(nested, array)
                },
                Err(e) => State::Error(make_parse_error(e))
            }
        },
        JSONToken::LeftBrace => {
            nested.push(NestedLevel::Array(array));
            let obj = <JSON as MakeJSON>::Object::new(&nested);
            State::ObjectBegin(nested, obj)
        },
        JSONToken::LeftBracket => {
            nested.push(NestedLevel::Array(array));
            let arr = <JSON as MakeJSON>::Array::new(&nested);
            State::ArrayBegin(nested, arr)
        },
        JSONToken::RightBracket => {
            let arr = JSON::make_array(array, &nested);
            end_nested(nested, arr)
        },
        unexpected => token_error(unexpected)
    }
}

pub fn match_array_with_value<JSON>(token:JSONToken, 
    nested: Vec<NestedLevel<JSON>>, 
    array: <JSON as MakeJSON>::Array) -> State<JSON>
    where JSON: MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error {
    match token {
        JSONToken::Comma => State::ArrayWithComma(nested, array),
        JSONToken::RightBracket => {
            let obj = JSON::make_array(array, &nested);
            end_nested(nested, obj)
        },
        unexpected => token_error(unexpected)
    }
}

pub fn match_array_with_comma<JSON>(token:JSONToken,
    mut nested: Vec<NestedLevel<JSON>>, 
    mut array: <JSON as MakeJSON>::Array) -> State<JSON>
    where JSON: MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error {
    match token {
        JSONToken::StringToken(s) => {
            array.add(JSON::make_string(s, &nested), &nested);
            State::ArrayWithValue(nested, array)
        },
        JSONToken::BoolToken(b) => {
            array.add(JSON::make_bool(b, &nested), &nested);
            State::ArrayWithValue(nested, array)
        },
        JSONToken::NullToken => {
            array.add(JSON::make_null(&nested), &nested);
            State::ArrayWithValue(nested, array)
        },
        JSONToken::NumberToken(s) => {
            match s.parse::<<JSON as MakeJSON>::Number>() {
                Ok(num) => {
                    array.add(JSON::make_number(num, &nested), &nested);
                    State::ArrayWithValue(nested, array)
                },
                Err(e) => State::Error(make_parse_error(e))
            }
        },
        JSONToken::LeftBrace => {
            nested.push(NestedLevel::Array(array));
            let obj = <JSON as MakeJSON>::Object::new(&nested);
            State::ObjectBegin(nested, obj)
        },
        JSONToken::LeftBracket => {
            nested.push(NestedLevel::Array(array));
            let arr = <JSON as MakeJSON>::Array::new(&nested);
            State::ArrayBegin(nested, arr)
        },
        unexpected => token_error(unexpected)
    }
}

pub fn match_end<JSON>(token: JSONToken) -> State<JSON>
    where JSON: MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error {
    token_error(token)
}

