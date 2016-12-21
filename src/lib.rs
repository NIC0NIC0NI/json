#[macro_use]
mod json_object;
mod from_json_str;
mod number;
mod convert;

use std::str::FromStr;

pub use convert::TryFromIterator;
pub use number::Number;
pub use json_object::JSON;
pub use from_json_str::ParseError;
use from_json_str::{TokenizeState, ParseState, from_char_stream};

/// Boxing makes `size_of::<TokenConsumer>()` much smaller, faster in parameter passing
type TokenConsumer = Box<ParseState>;
/// Boxing this makes more overhead than benefit
type Tokenizer = TokenizeState<TokenConsumer>;

impl TryFromIterator<char> for JSON {
    type Err = ParseError;
    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Err>
         where I: IntoIterator<Item=char>{
        from_char_stream::<Tokenizer,_>(iter.into_iter())
    }
}

impl FromStr for JSON {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_char_stream::<Tokenizer,_>(s.chars())
    }
}