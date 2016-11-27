mod tokenize;
mod parse;

pub use self::parse::State as ParseState;
pub use self::tokenize::State as TokenizeState;

use super::json_object::JSON;
use ::std::fmt::{Display,Formatter,Result as FmtResult};

pub type Error = String;

pub trait TokenConsumer {
    fn new() -> Self;
    fn consume(self, token: JSONToken) -> Self; 
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

impl Display for JSONToken {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &JSONToken::LeftBrace => write!(f, "{{"),
            &JSONToken::RightBrace => write!(f, "}}"),
            &JSONToken::LeftBracket => write!(f, "["),
            &JSONToken::RightBracket => write!(f, "]"),
            &JSONToken::Comma => write!(f, ","),
            &JSONToken::Colon => write!(f, ":"),
            &JSONToken::StringToken(ref s) => write!(f, "\"{}\"", s),
            &JSONToken::BoolToken(s) => write!(f, "{}", s),
            &JSONToken::IntToken(s) => write!(f, "{}", s),
            &JSONToken::FloatToken(s) => write!(f, "{}", s),
            &JSONToken::NullToken => write!(f, "null")
        }
    }
}

impl TokenizeState<ParseState> {
    pub fn into_json(self) -> Result<JSON, Error> {
        match self {
            TokenizeState::Out(consumer) => {
                match consumer {
                    ParseState::End(json) => Ok(json),
                    ParseState::Error(error) => Err(error),
                    ParseState::Begin => Err("Empty string".to_string()),
                    ParseState::ObjectBegin(_, _) | ParseState::ObjectWithName(_, _, _) | 
                        ParseState::ObjectWithColon(_, _, _) | ParseState::ObjectWithValue(_, _) |
                            ParseState::ObjectWithComma(_, _) => Err("Unmatched braces".to_string()),
                    ParseState::ArrayBegin(_, _) | ParseState::ArrayWithValue(_, _) |
                        ParseState::ArrayWithComma(_, _) => Err("Unmatched brackets".to_string()),
                }
            }
            TokenizeState::Error(msg) => Err(msg),
            _ => Err("Unmatched quotes".to_string())
        }
    }
}