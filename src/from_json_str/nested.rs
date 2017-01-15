
use ::std::str::FromStr;
use ::std::error::Error;
use ::std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use super::super::type_adapt::{MakeJSON, JSONObject, JSONArray};

/// Nested levels of JSON objects or arrays
pub enum NestedLevel <JSON> 
    where JSON : MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error {
    Array(<JSON as MakeJSON>::Array), 
    Object(<JSON as MakeJSON>::Object, String)
}

impl <JSON> NestedLevel <JSON> 
    where JSON : MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error {
    pub fn is_array(&self) -> bool {
        match self {
            &NestedLevel::Array(_) => true,
            _ => false
        }
    }
    pub fn is_object(&self) -> bool {
        match self {
            &NestedLevel::Object(_, _) => true,
            _ => false
        }
    }
    pub fn as_array(&self) -> Option<&<JSON as MakeJSON>::Array> {
        match self {
            &NestedLevel::Array(ref arr) => Some(&arr),
            _ => None
        }
    }
    pub fn as_object(&self) -> Option<&<JSON as MakeJSON>::Object> {
        match self {
            &NestedLevel::Object(ref obj, _) => Some(&obj),
            _ => None
        }
    }
    pub fn name_in_object(&self) -> Option<&str> {
        match self {
            &NestedLevel::Object(_, ref name) => Some(&name),
            _ => None
        }
    }
}

impl <JSON> Debug for NestedLevel <JSON> 
    where JSON : MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error + 'static,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON> + Debug,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON> + Debug{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &NestedLevel::Array(ref arr) => write!(f, "Array({:?})", arr),
            &NestedLevel::Object(ref obj, ref name) => write!(f, "Object({},{:?})", name, obj)
        }
    }
}

impl <JSON> Display for NestedLevel <JSON> 
    where JSON : MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err : Error + 'static,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON> + Display,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON> + Display{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &NestedLevel::Array(ref arr) => write!(f, "Array({})", arr),
            &NestedLevel::Object(ref obj, ref name) => write!(f, "Object({},{})", name, obj)
        }
    }
}