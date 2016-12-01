#[macro_use]
mod json_object;
mod from_json_str;

use std::str::FromStr;

pub use json_object::JSON;
pub use json_object::IntoJSON;
pub use from_json_str::ParseError;
use from_json_str::TokenizeState;
use from_json_str::ParseState;
use from_json_str::TryIntoJSON;
use from_json_str::from_char_stream;

/// Another version of `FromIterator`, may fail.
pub trait TryFromIterator<Item> : Sized{
    type Err;
    fn try_from_iter<I:IntoIterator<Item=Item>>(iter: I) -> Result<Self, Self::Err>;
}

/// Boxing makes `size_of::<TokenConsumer>()` much smaller, faster in parameter passing
type TokenConsumer = Box<ParseState>;
/// Boxing this makes more overhead than benefit
type Tokenizer = TokenizeState<TokenConsumer>;

impl TryFromIterator<char> for JSON {
    type Err = ParseError;
    fn try_from_iter<I:IntoIterator<Item=char>>(iter: I) -> Result<Self, ParseError>  {
        let result: Tokenizer = from_char_stream(iter.into_iter());
        result.try_into_json()
    }
}

impl FromStr for JSON {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        JSON::try_from_iter(s.chars())
    }
}
