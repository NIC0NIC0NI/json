#[macro_use]
mod json_tokens;
mod tokenize;
mod parse;
mod error;

use std::mem::replace;
pub use self::parse::State as ParseState;
pub use self::tokenize::State as TokenizeState;

use super::json_object::JSON;

/// Represents parse error
pub struct ParseError {
    description : String
}

pub trait Tokenizer {
    fn new() -> Self;
    fn tokenize(self, c: char) -> Self;
}

pub fn from_char_stream<T:Tokenizer, I:Iterator<Item=char>>(iter: I) -> T{
    let parse_state = T::new();
    iter.fold(parse_state, T::tokenize).tokenize(' ')
}

/// Used in self::tokenize
pub trait TokenConsumer {
    fn new() -> Self;
    fn consume(self, token: JSONToken) -> Self; 
}
impl <TC:TokenConsumer+Default> TokenConsumer for Box<TC> {
    fn new() -> Self {
        Box::new(TC::new())
    }
    fn consume(mut self, token: JSONToken) -> Self {
        /// Original code:
        /// ```Box::new((*self).parse_token(token))```
        /// optimized for memory reuse
        let placeholder = replace(&mut(*self), TC::default());  // use default to minimize construct overhead
        *self = placeholder.consume(token);
        self
    }
}

/// Used in super
pub trait TryIntoJSON {
    fn try_into_json(self) -> Result<JSON, ParseError>;
}
impl <I:TryIntoJSON> TryIntoJSON for Box<I> {
    fn try_into_json(self) -> Result<JSON, ParseError> {
        (*self).try_into_json()
    }
}

pub enum JSONToken {
    LeftBrace,
    RightBrace,
    LeftBracket, 
    RightBracket,
    Comma,
    Colon,
    StringToken(String),
    BoolToken(bool),
    IntToken(i64),
    FloatToken(f64),
    NullToken,
}

impl JSONToken {
    fn is_primitive_value(&self) -> bool {
        match self {
            &JSONToken::StringToken(_) | &JSONToken::BoolToken(_) | 
                &JSONToken::IntToken(_) | &JSONToken::FloatToken(_) | 
                    &JSONToken::NullToken => true,
            _ => false
        }
    }
    fn into_primitive_value(self) -> Option<JSON> {
        match self {
            JSONToken::StringToken(s) => Some(JSON::JSONString(s)),
            JSONToken::BoolToken(b) => Some(JSON::JSONBool(b)),
            JSONToken::IntToken(i) => Some(JSON::JSONInt(i)),
            JSONToken::FloatToken(f) => Some(JSON::JSONFloat(f)),
            JSONToken::NullToken => Some(JSON::JSONNull),
            _ => None
        }
    }
}
