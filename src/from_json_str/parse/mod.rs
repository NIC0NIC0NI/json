mod match_token;
#[cfg(test)]
mod test;

use super::{TokenConsumer, JSONToken, ParseError};
use super::super::type_adapt::{MakeJSON, JSONObject, JSONArray};
use super::super::convert::TryFrom;

use self::match_token::{match_begin, match_object_begin, match_object_with_name};
use self::match_token::{match_object_with_value, match_object_with_comma};
use self::match_token::{match_array_with_value, match_array_with_comma};
use self::match_token::{match_end, match_array_begin, match_object_with_colon};

pub enum NestedLevel <JSON> 
    where JSON : MakeJSON,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    Array(<JSON as MakeJSON>::Array), 
    Object(<JSON as MakeJSON>::Object, String)
}

pub enum State <JSON> 
    where JSON : MakeJSON,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    Begin,
    ObjectBegin(Vec<NestedLevel<JSON>>,
         <JSON as MakeJSON>::Object),
    ObjectWithName(Vec<NestedLevel<JSON>>, 
        <JSON as MakeJSON>::Object, String),
    ObjectWithColon(Vec<NestedLevel<JSON>>, 
        <JSON as MakeJSON>::Object, String),
    ObjectWithValue(Vec<NestedLevel<JSON>>, 
        <JSON as MakeJSON>::Object),
    ObjectWithComma(Vec<NestedLevel<JSON>>, 
        <JSON as MakeJSON>::Object),
    ArrayBegin(Vec<NestedLevel<JSON>>, 
        <JSON as MakeJSON>::Array),
    ArrayWithValue(Vec<NestedLevel<JSON>>, 
        <JSON as MakeJSON>::Array),
    ArrayWithComma(Vec<NestedLevel<JSON>>, 
        <JSON as MakeJSON>::Array),
    End(JSON),
    Error(ParseError),
}

impl <JSON> State <JSON> 
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
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

impl <JSON> TokenConsumer for State <JSON> 
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    fn new() -> Self {
        State::Begin
    }
    fn consume(self, token: JSONToken) -> Self {
        self.parse_token(token)
    }
}

impl <JSON> Default for State <JSON> 
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    fn default() -> Self {
        State::Begin
    }
}

impl <JSON> TryFrom<State<JSON>> for JSON 
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    type Err = ParseError;
    fn try_from(s: State<JSON>) -> Result<JSON, Self::Err> {
        match s {
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
