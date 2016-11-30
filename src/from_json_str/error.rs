use super::ParseError;
use ::std::error::Error;
use ::std::fmt::{Display,Debug,Formatter,Result as FmtResult};

impl From<String> for ParseError {
    fn from(description: String) -> Self {
        ParseError {description : description}
    }
}

impl <'a> From<&'a str> for ParseError {
    fn from(description: &'a str) -> Self {
        ParseError {description : description.to_string()}
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.description
    }
    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description)
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self.description)
    }
}