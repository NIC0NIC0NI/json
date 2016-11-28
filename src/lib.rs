#[macro_export]
macro_rules! json_object {
    ( [$($item:tt),+] ) => {{
        let mut vector = Vec::new();
        $(
            vector.push(json_object!($item));
        )*
        $crate::JSON::JSONArray(vector)
    }};
    ( [] ) => {{
        $crate::JSON::JSONArray(Vec::new())
    }};
    ( {$($name:ident : $value:tt),+} ) => {{
        let mut hash_map = ::std::collections::HashMap::new();
        $(
            hash_map.insert(stringify!($name).to_string(), json_object!($value));
        )*
        $crate::JSON::JSONObject(hash_map)
    }};
    ( {} ) => {{
        $crate::JSON::JSONObject(::std::collections::HashMap::new())
    }};
    (null) => {
        $crate::JSON::JSONNull
    };
    ($x:expr) => {
        $crate::JSONValue::to_json($x)
    };
}

mod json_object;
mod from_json_str;

pub use std::str::FromStr;
pub use std::collections::HashMap;

pub use json_object::JSON;
pub use json_object::JSONValue;
use from_json_str::TokenizeState;
use from_json_str::ParseState;
use from_json_str::IntoJSON;
use from_json_str::tokenize_str;

/// Boxing makes `size_of::<TokenConsumer>()` much smaller, faster in parameter passing
type TokenConsumer = Box<ParseState>;
/// Boxing this makes more overhead than benefit
type Tokenizer = TokenizeState<TokenConsumer>;

impl FromStr for JSON {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = tokenize_str::<Tokenizer>(s);
        result.into_json()
    }
}
