//! This module is used for testing.
//! Since tokens are invisible outside the crate.

#[macro_use]
mod macros;
mod token_value;

use super::JSONToken;

pub trait TokenValue {
     fn to_token(self) -> JSONToken;
}

