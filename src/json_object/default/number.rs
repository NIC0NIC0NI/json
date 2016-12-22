
use ::std::fmt::{Display,Formatter,Result as FmtResult};
use ::std::str::FromStr;
use super::super::super::ParseError;
use super::super::super::from_json_str::make_parse_error;
use super::super::super::convert::FromPremitive;

#[derive(Debug, Clone, Copy)]
pub enum Number {
    Int(i64), Float(f64), UInt(u64)
}

impl Display for Number {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match self {
            &Number::Int(i) => write!(fmt, "{}", i),
            &Number::Float(f) => write!(fmt, "{}", f),
            &Number::UInt(u) => write!(fmt, "{}", u),
        }
    }
}

fn eq_su(signed: i64, unsigned: u64) -> bool {
    if signed < 0 {
        false
    } else if unsigned > i64::max_value() as u64 {
        false
    } else {
        signed == unsigned as i64
    }
}

impl PartialEq for Number {
    fn eq(&self, another: &Number) -> bool {
        match self {
            &Number::Int(x) => {
                match another {
                    &Number::Int(y) => x == y,
                    &Number::UInt(y) => eq_su(x,y),
                    &Number::Float(y) => x as f64 == y,
                }
            },
            &Number::UInt(x) => {
                match another {
                    &Number::Int(y) => eq_su(y,x),
                    &Number::UInt(y) => x == y,
                    &Number::Float(y) => x as f64 == y,
                }
            },
            &Number::Float(x) => {
                match another {
                    &Number::Int(y) => x == y as f64,
                    &Number::UInt(y) => x == y as f64,
                    &Number::Float(y) => x == y,
                }
            }
        }
    }
}


impl FromStr for Number {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('.') {
            match s.parse::<f64>() {
                Ok(f) => Ok(Number::Float(f)),
                Err(e) => Err(make_parse_error(e))
            }
        } else if let Ok(i) = s.parse::<i64>() {
            Ok(Number::Int(i))
        } else {
            match s.parse::<f64>() {
                Ok(f) => Ok(Number::Float(f)),
                Err(e) => Err(make_parse_error(e))
            }
        }
    }
}



macro_rules! register_int_type {
    ($t:ty) => {
        impl FromPremitive<$t> for Number {
            fn from_premitive(n: $t) -> Number {
                Number::Int(n as i64)
            }
        }
    }
}

register_int_type!{i8}
register_int_type!{i16}
register_int_type!{i32}
register_int_type!{i64}
register_int_type!{u8}
register_int_type!{u16}
register_int_type!{u32}

macro_rules! register_float_type {
    ($t:ty) => {
        impl FromPremitive<$t> for Number {
            fn from_premitive(n: $t) -> Number {
                Number::Float(n as f64)
            }
        }
    }
}

register_float_type!{f32}
register_float_type!{f64}