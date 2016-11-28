#[macro_use]
mod json_tokens;
mod tokenize;
mod parse;

use std::mem::swap;
pub use self::parse::State as ParseState;
pub use self::tokenize::State as TokenizeState;

use super::json_object::JSON;

pub type Error = String;

pub trait Tokenizer {
    fn new() -> Self;
    fn tokenize(self, c: char) -> Self;
}

pub fn tokenize_str<T:Tokenizer>(s: &str) -> T {
    let parse_state = T::new();
    s.chars().fold(parse_state, T::tokenize).tokenize(' ')
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
        let mut placeholder = TC::default();  // use default to minimize construct overhead
        swap(&mut(*self), &mut placeholder);
        *self = placeholder.consume(token);
        self
    }
}

/// Used in super
pub trait IntoJSON {
    fn into_json(self) -> Result<JSON, Error>;
}
impl <I:IntoJSON> IntoJSON for Box<I> {
    fn into_json(self) -> Result<JSON, Error> {
        (*self).into_json()
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
