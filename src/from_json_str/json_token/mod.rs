#[cfg(test)]
#[macro_use]
pub mod test;

use ::std::fmt::{Display, Formatter, Result as FmtResult};

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
    NumberToken(String),
    NullToken,
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
            &JSONToken::NumberToken(ref n) => write!(f, "{}", n),
            &JSONToken::NullToken => write!(f, "null")
        }
    }
}
