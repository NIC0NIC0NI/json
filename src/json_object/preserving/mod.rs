mod json;

use ::std::fmt::Display;
use super::super::FromPremitive;

#[derive(PartialEq, Debug, Clone)]
pub enum PreservingJSON {
    Bool(bool),
    Number(String),  // to avoid overflow, just left strings
    String(String),
    Array(Vec<PreservingJSON>),
    Object(Vec<(String, PreservingJSON)>), // to preserve order
    Null
}

impl <P:Display> FromPremitive<P> for String {
    fn from_premitive(p: P) -> String {
        p.to_string()
    }
}
