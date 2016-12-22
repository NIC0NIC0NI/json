use super::Number;
use ::std::fmt::{Display,Formatter,Result as FmtResult};


impl Display for Number {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match self {
            &Number::Int(i) => write!(fmt, "{}", i),
            &Number::Float(f) => write!(fmt, "{}", f),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, another: &Number) -> bool {
        match self {
            &Number::Int(x) => {
                match another {
                    &Number::Int(y) => x == y,
                    &Number::Float(y) => x as f64 == y,
                }
            },
            &Number::Float(x) => {
                match another {
                    &Number::Int(y) => x == y as f64,
                    &Number::Float(y) => x == y,
                }
            }
        }
    }
}
