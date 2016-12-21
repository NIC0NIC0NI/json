mod match_token;
#[cfg(test)]
mod test;

use super::{TokenConsumer, JSONToken, TryIntoJSON, ParseError};
use super::super::json_object::{JSON, JSONObject, JSONArray};

use self::match_token::{match_begin, match_object_begin, match_object_with_name};
use self::match_token::{match_object_with_value, match_object_with_comma};
use self::match_token::{match_array_with_value, match_array_with_comma};
use self::match_token::{match_end, match_array_begin, match_object_with_colon};

pub enum NestedLevel {
    Array(JSONArray), Object(JSONObject, String)
}

pub enum State {
    Begin,
    ObjectBegin(Vec<NestedLevel>, JSONObject),
    ObjectWithName(Vec<NestedLevel>, JSONObject, String),
    ObjectWithColon(Vec<NestedLevel>, JSONObject, String),
    ObjectWithValue(Vec<NestedLevel>, JSONObject),
    ObjectWithComma(Vec<NestedLevel>, JSONObject),
    ArrayBegin(Vec<NestedLevel>, JSONArray),
    ArrayWithValue(Vec<NestedLevel>, JSONArray),
    ArrayWithComma(Vec<NestedLevel>, JSONArray),
    End(JSON),
    Error(ParseError),
}

impl State {
    fn parse_token(self, token: JSONToken) -> Self{
        match self {
            State::Begin => 
                match_begin(token),
            State::ObjectBegin(nested, object) =>
                match_object_begin(token, nested, object),
            State::ObjectWithName(nested, object, name) =>
                match_object_with_name(token, nested, object, name),
            State::ObjectWithColon(nested, object, name) =>
                match_object_with_colon(token, nested, object, name),
            State::ObjectWithValue(nested, object) =>
                match_object_with_value(token, nested, object),
            State::ObjectWithComma(nested, object) =>
                match_object_with_comma(token, nested, object),
            State::ArrayBegin(nested, array) =>
                match_array_begin(token, nested, array),
            State::ArrayWithValue(nested, array) => 
                match_array_with_value(token, nested, array),
            State::ArrayWithComma(nested, array) => 
                match_array_with_comma(token, nested, array),
            State::End(_) => 
                match_end(token),
            error => error,
        }
    }
}

impl TokenConsumer for State {
    fn new() -> Self {
        State::Begin
    }
    fn consume(self, token: JSONToken) -> Self {
        self.parse_token(token)
    }
}

impl Default for State {
    fn default() -> Self {
        State::Begin
    }
}

impl TryIntoJSON for State {
    fn try_into_json(self) -> Result<JSON, ParseError> {
        match self {
            State::End(json) => Ok(json),
            State::Error(error) => Err(error),
            State::Begin => Err("Empty string".into()),
            State::ObjectBegin(_, _) | State::ObjectWithName(_, _, _) | 
                State::ObjectWithColon(_, _, _) | State::ObjectWithValue(_, _) |
                    State::ObjectWithComma(_, _) => Err("Unmatched braces".into()),
            State::ArrayBegin(_, _) | State::ArrayWithValue(_, _) |
                State::ArrayWithComma(_, _) => Err("Unmatched brackets".into()),
        }
    }
}
