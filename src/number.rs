

use ::std::str::FromStr;
use ::std::fmt::{Display,Formatter,Result as FmtResult};

/// Represents an integer or a floating point number
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Number {
    Int(i64), Float(f64)
}


impl Number {
    pub fn is_integer(self) -> bool {
        match self {
            Number::Int(_) => true,
            _ => false,
        }
    }
    pub fn is_float(self) -> bool {
        !self.is_integer()
    }
    pub fn as_int(self) -> i64 {
        match self {
            Number::Int(i) => i,
            Number::Float(f) => f as i64,
        }
    }
    pub fn as_float(self) -> f64 {
        match self {
            Number::Int(i) => i as f64,
            Number::Float(f) => f,
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Number::Int(i) => write!(f, "{}", i),
            Number::Float(fp) => write!(f, "{}", fp)
        }
    }
}

impl FromStr for Number {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i) = i64::from_str(s) {
            Ok(Number::Int(i))
        } else if let Ok(f) = f64::from_str(s) {
            Ok(Number::Float(f))
        } else {
            Err(())
        }
    }
}