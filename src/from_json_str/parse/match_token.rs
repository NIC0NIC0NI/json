//! Describes the state transition

use ::std::str::FromStr;
use ::std::error::Error;

use super::super::super::type_adapt::{MakeJSON, JSONObject, JSONArray};
use super::super::{JSONToken, make_parse_error};
use super::{NestedLevel, State};

fn token_error<JSON>(token: JSONToken) -> State <JSON> 
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Number : FromStr,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error + 'static,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    State::Error(format!("Unexpected token {}", token).into())
}

fn end_nested<JSON>(mut nested: Vec<NestedLevel<JSON>>, json: JSON) -> State<JSON> 
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Number : FromStr,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error + 'static,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    if let Some(up) = nested.pop() {
        match up {
            NestedLevel::Array(mut array) => {
                array.add(json);
                State::ArrayWithValue(nested, array)
            },
            NestedLevel::Object(mut object, name) => {
                object.add(name, json);
                State::ObjectWithValue(nested, object)
            }
        }
    } else {
        State::End(json)
    }
}

pub fn match_begin<JSON>(token:JSONToken) -> State<JSON> 
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Number : FromStr,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error + 'static,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    match token {
        JSONToken::LeftBrace => State::ObjectBegin(Vec::new(), JSONObject::new()),
        JSONToken::LeftBracket => State::ArrayBegin(Vec::new(), JSONArray::new()),
        JSONToken::StringToken(s) => State::End(JSON::make_string(s)),
        JSONToken::BoolToken(b) => State::End(JSON::make_bool(b)),
        JSONToken::NumberToken(s) => {
            match s.parse::<<JSON as MakeJSON>::Number>() {
                Ok(n) => State::End(JSON::make_number(n)),
                Err(e) => State::Error(make_parse_error(e))
            }
        },
        JSONToken::NullToken => State::End(JSON::make_null()),
        unexpected => token_error(unexpected)
    }
}

pub fn match_object_begin<JSON>(token:JSONToken, 
    nested: Vec<NestedLevel<JSON>>, 
    object: <JSON as MakeJSON>::Object) -> State<JSON>
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Number : FromStr,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error + 'static,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    match token {
        JSONToken::StringToken(name) => State::ObjectWithName(nested, object, name),
        JSONToken::RightBrace => end_nested(nested, JSON::make_object(object)),
        unexpected => token_error(unexpected)
    }
}

pub fn match_object_with_name<JSON>(token:JSONToken, 
    nested: Vec<NestedLevel<JSON>>, 
    object: <JSON as MakeJSON>::Object, name: String) -> State<JSON> 
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Number : FromStr,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error + 'static,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
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
          <JSON as MakeJSON>::Number : FromStr,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error + 'static,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    match token {
        JSONToken::StringToken(s) => {
            object.add(name, JSON::make_string(s));
            State::ObjectWithValue(nested, object)
        },
        JSONToken::BoolToken(b) => {
            object.add(name, JSON::make_bool(b));
            State::ObjectWithValue(nested, object)
        },
        JSONToken::NullToken => {
            object.add(name, JSON::make_null());
            State::ObjectWithValue(nested, object)
        },
        JSONToken::NumberToken(s) => {
            match s.parse::<<JSON as MakeJSON>::Number>() {
                Ok(num) => {
                    object.add(name, JSON::make_number(num));
                    State::ObjectWithValue(nested, object)
                },
                Err(e) => State::Error(make_parse_error(e))
            }
        },
        JSONToken::LeftBrace => {
            nested.push(NestedLevel::Object(object, name));
            State::ObjectBegin(nested, <JSON as MakeJSON>::Object::new())
        },
        JSONToken::LeftBracket => {
            nested.push(NestedLevel::Object(object, name));
            State::ArrayBegin(nested, <JSON as MakeJSON>::Array::new())
        },
        unexpected => token_error(unexpected)
    }
    
}

pub fn match_object_with_value<JSON>(token:JSONToken, 
    nested: Vec<NestedLevel<JSON>>, 
    object: <JSON as MakeJSON>::Object) -> State<JSON>
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Number : FromStr,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error + 'static,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    match token {
        JSONToken::Comma => State::ObjectWithComma(nested, object),
        JSONToken::RightBrace => end_nested(nested, JSON::make_object(object)),
        unexpected => token_error(unexpected)
    }
}

pub fn match_object_with_comma<JSON>(token:JSONToken, 
    nested: Vec<NestedLevel<JSON>>, 
    object: <JSON as MakeJSON>::Object) -> State<JSON>
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Number : FromStr,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error + 'static,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    match token {
        JSONToken::StringToken(name) => State::ObjectWithName(nested, object, name),
        unexpected => token_error(unexpected)
    }
}

pub fn match_array_begin<JSON>(token:JSONToken, 
    mut nested: Vec<NestedLevel<JSON>>, 
    mut array: <JSON as MakeJSON>::Array) -> State<JSON>
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Number : FromStr,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error + 'static,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    match token {
        JSONToken::StringToken(s) => {
            array.add(JSON::make_string(s));
            State::ArrayWithValue(nested, array)
        },
        JSONToken::BoolToken(b) => {
            array.add(JSON::make_bool(b));
            State::ArrayWithValue(nested, array)
        },
        JSONToken::NullToken => {
            array.add(JSON::make_null());
            State::ArrayWithValue(nested, array)
        },
        JSONToken::NumberToken(s) => {
            match s.parse::<<JSON as MakeJSON>::Number>() {
                Ok(num) => {
                    array.add(JSON::make_number(num));
                    State::ArrayWithValue(nested, array)
                },
                Err(e) => State::Error(make_parse_error(e))
            }
        },
        JSONToken::LeftBrace => {
            nested.push(NestedLevel::Array(array));
            State::ObjectBegin(nested, JSONObject::new())
        },
        JSONToken::LeftBracket => {
            nested.push(NestedLevel::Array(array));
            State::ArrayBegin(nested, JSONArray::new())
        },
        JSONToken::RightBracket => end_nested(nested, JSON::make_array(array)),
        unexpected => token_error(unexpected)
    }
}

pub fn match_array_with_value<JSON>(token:JSONToken, 
    nested: Vec<NestedLevel<JSON>>, 
    array: <JSON as MakeJSON>::Array) -> State<JSON>
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Number : FromStr,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error + 'static,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    match token {
        JSONToken::Comma => State::ArrayWithComma(nested, array),
        JSONToken::RightBracket => end_nested(nested, JSON::make_array(array)),
        unexpected => token_error(unexpected)
    }
}

pub fn match_array_with_comma<JSON>(token:JSONToken,
    mut nested: Vec<NestedLevel<JSON>>, 
    mut array: <JSON as MakeJSON>::Array) -> State<JSON>
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Number : FromStr,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error + 'static + 'static,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    match token {
        JSONToken::StringToken(s) => {
            array.add(JSON::make_string(s));
            State::ArrayWithValue(nested, array)
        },
        JSONToken::BoolToken(b) => {
            array.add(JSON::make_bool(b));
            State::ArrayWithValue(nested, array)
        },
        JSONToken::NullToken => {
            array.add(JSON::make_null());
            State::ArrayWithValue(nested, array)
        },
        JSONToken::NumberToken(s) => {
            match s.parse::<<JSON as MakeJSON>::Number>() {
                Ok(num) => {
                    array.add(JSON::make_number(num));
                    State::ArrayWithValue(nested, array)
                },
                Err(e) => State::Error(make_parse_error(e))
            }
        },
        JSONToken::LeftBrace => {
            nested.push(NestedLevel::Array(array));
            State::ObjectBegin(nested, <JSON as MakeJSON>::Object::new())
        },
        JSONToken::LeftBracket => {
            nested.push(NestedLevel::Array(array));
            State::ArrayBegin(nested, <JSON as MakeJSON>::Array::new())
        },
        unexpected => token_error(unexpected)
    }
}

pub fn match_end<JSON>(token: JSONToken) -> State<JSON>
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Number : FromStr,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error + 'static,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    token_error(token)
}

