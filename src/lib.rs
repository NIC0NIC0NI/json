mod from_json;
mod json_object;

pub use std::str::FromStr;
pub use std::collections::HashMap;

pub use json_object::JSON;
pub use json_object::JSONValue;
use from_json::TokenizeState;
use from_json::ParseState;

impl FromStr for JSON {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_state:TokenizeState<ParseState> = TokenizeState::new();
        let result = s.chars().fold(parse_state, TokenizeState::tokenize).tokenize(' ');
        result.into_json()
    }
}

macro_rules! json_object {
    // make JSON array by [item0, item1, .., itemn]
    ( [$($item:tt),+] ) => {{
        let mut vector = Vec::new();
        $(
            vector.push(json_object!($item));
        )*
        JSON::JSONArray(vector)
    }};
    // specialized in order to eliminate "unused mut" warning
    ( [] ) => {{
        JSON::JSONArray(Vec::new())
    }};


    // make JSON object by {name0:value0, name1:value1, .., namen:valuen}
    // note that $name is not quoted
    ( {$($name:ident : $value:tt),+} ) => {{
        let mut hash_map = HashMap::new();
        $(
            hash_map.insert(stringify!($name).to_string(), json_object!($value));
        )*
        JSON::JSONObject(hash_map)
    }};
    // specialized in order to eliminate "unused mut" warning
    ( {} ) => {{
        JSON::JSONObject(HashMap::new())
    }};


    // Rust have no 'null's
    (null) => {
        JSON::JSONNull
    };
    // primitive values
    ($x:expr) => {
        $x.to_json()
    };
}

#[cfg(test)]
mod test;