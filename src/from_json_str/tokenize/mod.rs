mod match_char;
#[cfg(test)]
mod test;

//use std::mem::swap;

use super::TokenConsumer;
use super::Tokenizer;
use super::IntoJSON;
use super::Error as TokenizeError;
use ::JSON;

use self::match_char::match_in_string;
use self::match_char::match_in_string_escape;
use self::match_char::match_in_string_escape_unicode;
use self::match_char::match_in_value;
use self::match_char::match_out;

pub enum State <Consumer> {
    InString (Consumer, String),
    InStringEscape (Consumer, String),
    InStringEscapeUnicode (Consumer, String, String),
    InValue (Consumer, String),
    Out (Consumer),
    Error (TokenizeError)
}

impl <TC:TokenConsumer> Tokenizer for State <TC> {
    fn new() -> Self {
        State::Out (TC::new())
    }
    fn tokenize(self, c: char) -> Self {
        match self {
            State::InString(consumer, word) => 
                match_in_string(c, consumer, word),
            State::InStringEscape(consumer, word) => 
                match_in_string_escape(c, consumer, word),
            State::InStringEscapeUnicode(consumer, word, codepoint) => 
                match_in_string_escape_unicode(c, consumer, word, codepoint),
            State::InValue(consumer, word) => 
                match_in_value(c, consumer, word),
            State::Out(consumer) => 
                match_out(c, consumer),
            error => error,
        }
    }
}

impl <I:IntoJSON>  IntoJSON for State<I> {
    fn into_json(self) -> Result<JSON, TokenizeError> {
        match self {
            State::Out(i) => i.into_json(),
            State::Error(msg) => Err(msg),
            _ => Err("Unmatched quotes".to_string())
        }
    }
}