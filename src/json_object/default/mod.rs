mod json;
mod number;
use super::super::FromJSONStr;

use ::std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Number {
    Int(i64), Float(f64)
}

pub type Map = HashMap<String, DefaultJSON>;

#[derive(PartialEq, Debug, Clone)]
pub enum DefaultJSON {
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<DefaultJSON>),
    Object(Map),
    Null
}

impl FromStr for DefaultJSON {
    type Err = <DefaultJSON as FromJSONStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <DefaultJSON as FromJSONStr>::from_json_str(s)
    }
}
