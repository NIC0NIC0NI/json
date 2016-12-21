

use ::std::str::FromStr;
use ::std::fmt::{Display,Formatter,Result as FmtResult};

/// Represents an integer or a floating point number
#[derive(Debug, Clone, Copy)]
pub enum Number {
    Int(i64), UInt(u64), Float(f64)
}


impl Number {
    pub fn is_int(self) -> bool {
        match self {
            Number::Int(_) => true,
            _ => false,
        }
    }
    pub fn is_uint(self) -> bool {
        match self {
            Number::UInt(_) => true,
            _ => false,
        }
    }
    pub fn is_float(self) -> bool {
        match self {
            Number::Float(_) => true,
            _ => false,
        }
    }
    pub fn as_int(self) -> i64 {
        match self {
            Number::Int(i) => i,
            Number::UInt(u) => u as i64,
            Number::Float(f) => f as i64,
        }
    }
    pub fn as_uint(self) -> u64 {
        match self {
            Number::Int(i) => i as u64,
            Number::UInt(u) => u,
            Number::Float(f) => f as u64,
        }
    }
    pub fn as_float(self) -> f64 {
        match self {
            Number::Int(i) => i as f64,
            Number::UInt(u) => u as f64,
            Number::Float(f) => f,
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, another: &Number) -> bool {
        if self.is_float() || another.is_float() {
            self.as_float() == another.as_float()
        } else if self.is_int() || another.is_int() {
            self.as_int() == another.as_int()
        } else {
            self.as_uint() == another.as_uint()
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Number::Int(i) => write!(f, "{}", i),
            Number::UInt(u) => write!(f, "{}", u),
            Number::Float(fp) => write!(f, "{}", fp)
        }
    }
}

impl FromStr for Number {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i) = i64::from_str(s) {
            Ok(Number::Int(i))
        } else if let Ok(u) = u64::from_str(s) {
            Ok(Number::UInt(u))
        } else if let Ok(f) = f64::from_str(s) {
            Ok(Number::Float(f))
        } else {
            Err(())
        }
    }
}

