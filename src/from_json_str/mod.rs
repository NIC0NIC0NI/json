#[macro_use]
#[cfg(test)]
mod json_token;
mod tokenize;
mod parse;
mod error;

use std::mem::replace;
use ::std::fmt::{Display, Formatter, Result as FmtResult};

pub use self::parse::State as ParseState;
pub use self::tokenize::State as TokenizeState;

use super::json_object::{JSON, JSONNumber};

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

#[derive(PartialEq, Debug)]
pub enum JSONToken {
    LeftBrace,
    RightBrace,
    LeftBracket, 
    RightBracket,
    Comma,
    Colon,
    StringToken(String),
    BoolToken(bool),
    NumberToken(JSONNumber),
    NullToken,
}

impl JSONToken {
    fn is_primitive_value(&self) -> bool {
        match self {
            &JSONToken::StringToken(_) | &JSONToken::BoolToken(_) | 
                &JSONToken::NumberToken(_) |
                    &JSONToken::NullToken => true,
            _ => false
        }
    }
    fn into_primitive_value(self) -> Option<JSON> {
        match self {
            JSONToken::StringToken(s) => Some(JSON::String(s)),
            JSONToken::BoolToken(b) => Some(JSON::Bool(b)),
            JSONToken::NumberToken(n) => Some(JSON::Number(n)),
            JSONToken::NullToken => Some(JSON::Null),
            _ => None
        }
    }
}

impl Display for JSONToken {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &JSONToken::LeftBrace => write!(f, "{{"),
            &JSONToken::RightBrace => write!(f, "}}"),
            &JSONToken::LeftBracket => write!(f, "["),
            &JSONToken::RightBracket => write!(f, "]"),
            &JSONToken::Comma => write!(f, ", "),
            &JSONToken::Colon => write!(f, ": "),
            &JSONToken::StringToken(ref s) => write!(f, "\"{}\"", s),
            &JSONToken::BoolToken(s) => write!(f, "{}", s),
            &JSONToken::NumberToken(n) => write!(f, "{}", n),
            &JSONToken::NullToken => write!(f, "null")
        }
    }
}
