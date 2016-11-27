mod match_token;

use super::TokenConsumer;
use super::JSONToken;
use super::Error as ParseError;
use super::super::json_object::JSON;
use super::super::json_object::NameValuePair;

use self::match_token::match_begin;
use self::match_token::match_object_begin;
use self::match_token::match_object_with_name;
use self::match_token::match_object_with_colon;
use self::match_token::match_object_with_value;
use self::match_token::match_object_with_comma;
use self::match_token::match_array_begin;
use self::match_token::match_array_with_value;
use self::match_token::match_array_with_comma;
use self::match_token::match_end;

pub enum NestedLevel {
    Array(Vec<JSON>), Object(NameValuePair, String)
}

pub enum State {
    Begin,
    ObjectBegin(Vec<NestedLevel>, NameValuePair),
    ObjectWithName(Vec<NestedLevel>, NameValuePair, String),
    ObjectWithColon(Vec<NestedLevel>, NameValuePair, String),
    ObjectWithValue(Vec<NestedLevel>, NameValuePair),
    ObjectWithComma(Vec<NestedLevel>, NameValuePair),
    ArrayBegin(Vec<NestedLevel>, Vec<JSON>),
    ArrayWithValue(Vec<NestedLevel>, Vec<JSON>),
    ArrayWithComma(Vec<NestedLevel>, Vec<JSON>),
    End(JSON),
    Error(ParseError)
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

impl TokenConsumer for Box<State> {
    fn new() -> Self {
        Box::new(State::Begin)
    }
    fn consume(self, token: JSONToken) -> Self {
        Box::new((*self).parse_token(token))
    }
}

