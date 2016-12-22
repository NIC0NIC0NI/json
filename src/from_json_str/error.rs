
use ::std::error::Error;
use ::std::fmt::{Display,Debug,Formatter,Result as FmtResult};

/// Represents parse error
pub enum ParseError {
    Generated(String),
    Caused(Box<Error>)
}

pub fn make_parse_error<E:Error + 'static>(e: E) -> ParseError {
    ParseError::Caused(Box::new(e))
}

impl From<String> for ParseError {
    fn from(description: String) -> Self {
        ParseError::Generated(description)
    }
}

impl <'a> From<&'a str> for ParseError {
    fn from(description: &'a str) -> Self {
        ParseError::Generated(description.to_string())
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match self{
            &ParseError::Generated(ref s) => s,
            &ParseError::Caused(ref c) => c.description()
        }
    }
    fn cause(&self) -> Option<&Error> {
        match self{
            &ParseError::Generated(_) => None,
            &ParseError::Caused(ref c) => Some(c.as_ref())
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}