use super::super::super::json_object::JSON;
use super::super::super::json_object::NameValuePair;
use super::super::JSONToken;
use super::NestedLevel;
use super::State;

fn token_error(token: JSONToken) -> State {
    State::Error(format!("Unexpected token {}", token).into())
}

fn end_nested(mut nested: Vec<NestedLevel>, json: JSON) -> State {
    if let Some(up) = nested.pop() {
        match up {
            NestedLevel::Array(mut array) => {
                array.push(json);
                State::ArrayWithValue(nested, array)
            },
            NestedLevel::Object(mut object, name) => {
                object.insert(name, json);
                State::ObjectWithValue(nested, object)
            }
        }
    } else {
        State::End(json)
    }
}

pub fn match_begin(token:JSONToken) -> State{
    match token {
        JSONToken::LeftBrace => State::ObjectBegin(Vec::new(), NameValuePair::new()),
        JSONToken::LeftBracket => State::ArrayBegin(Vec::new(), Vec::new()),
        JSONToken::StringToken(s) => State::End(JSON::JSONString(s)),
        JSONToken::BoolToken(b) => State::End(JSON::JSONBool(b)),
        JSONToken::IntToken(i) => State::End(JSON::JSONInt(i)),
        JSONToken::FloatToken(f) => State::End(JSON::JSONFloat(f)),
        JSONToken::NullToken => State::End(JSON::JSONNull),
        unexpected => token_error(unexpected)
    }
}

pub fn match_object_begin(token:JSONToken, nested: Vec<NestedLevel>, object: NameValuePair) -> State{
    match token {
        JSONToken::StringToken(name) => State::ObjectWithName(nested, object, name),
        JSONToken::RightBrace => end_nested(nested, JSON::JSONObject(object)),
        unexpected => token_error(unexpected)
    }
}

pub fn match_object_with_name(token:JSONToken, nested: Vec<NestedLevel>, object: NameValuePair, name: String) -> State{
    match token {
        JSONToken::Colon => State::ObjectWithColon(nested, object, name),
        unexpected => token_error(unexpected)
    }
}

pub fn match_object_with_colon(token:JSONToken, mut nested: Vec<NestedLevel>, mut object: NameValuePair, name: String) -> State{
    if token.is_primitive_value() {
        let value = token.into_primitive_value().unwrap();
        object.insert(name, value);
        State::ObjectWithValue(nested, object)
    } else {
        match token {
            JSONToken::LeftBrace => {
                nested.push(NestedLevel::Object(object, name));
                State::ObjectBegin(nested, NameValuePair::new())
            },
            JSONToken::LeftBracket => {
                nested.push(NestedLevel::Object(object, name));
                State::ArrayBegin(nested, Vec::new())
            },
            unexpected => token_error(unexpected)
        }
    }
}

pub fn match_object_with_value(token:JSONToken, nested: Vec<NestedLevel>, object: NameValuePair) -> State{
    match token {
        JSONToken::Comma => State::ObjectWithComma(nested, object),
        JSONToken::RightBrace => end_nested(nested, JSON::JSONObject(object)),
        unexpected => token_error(unexpected)
    }
}

pub fn match_object_with_comma(token:JSONToken, nested: Vec<NestedLevel>, object: NameValuePair) -> State{
    match token {
        JSONToken::StringToken(name) => State::ObjectWithName(nested, object, name),
        unexpected => token_error(unexpected)
    }
}

pub fn match_array_begin(token:JSONToken, mut nested: Vec<NestedLevel>, mut array: Vec<JSON>) -> State{
    if token.is_primitive_value() {
        let value = token.into_primitive_value().unwrap();
        array.push(value);
        State::ArrayWithValue(nested, array)
    } else {
        match token {
            JSONToken::LeftBrace => {
                nested.push(NestedLevel::Array(array));
                State::ObjectBegin(nested, NameValuePair::new())
            },
            JSONToken::LeftBracket => {
                nested.push(NestedLevel::Array(array));
                State::ArrayBegin(nested, Vec::new())
            },
            JSONToken::RightBracket => end_nested(nested, JSON::JSONArray(array)),
            unexpected => token_error(unexpected)
        }
    }
}

pub fn match_array_with_value(token:JSONToken, nested: Vec<NestedLevel>, array: Vec<JSON>) -> State{
    match token {
        JSONToken::Comma => State::ArrayWithComma(nested, array),
        JSONToken::RightBracket => end_nested(nested, JSON::JSONArray(array)),
        unexpected => token_error(unexpected)
    }
}

pub fn match_array_with_comma(token:JSONToken, mut nested: Vec<NestedLevel>, mut array: Vec<JSON>) -> State{
    if token.is_primitive_value() {
        let value = token.into_primitive_value().unwrap();
        array.push(value);
        State::ArrayWithValue(nested, array)
    } else {
        match token {
            JSONToken::LeftBrace => {
                nested.push(NestedLevel::Array(array));
                State::ObjectBegin(nested, NameValuePair::new())
            },
            JSONToken::LeftBracket => {
                nested.push(NestedLevel::Array(array));
                State::ArrayBegin(nested, Vec::new())
            },
            unexpected => token_error(unexpected)
        }
    }
}

pub fn match_end(token: JSONToken) -> State{
    token_error(token)
}

