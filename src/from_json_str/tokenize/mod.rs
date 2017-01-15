mod match_char;
#[cfg(test)]
mod test;

use ::std::str::FromStr;
use ::std::error::Error;

use super::{TokenConsumer, Tokenizer};
use super::ParseError as TokenizeError;
use super::super::convert::{TryFrom, TryInto};
use super::super::type_adapt::{MakeJSON, JSONObject, JSONArray};
use self::match_char::{match_in_string, match_in_string_escape_unicode};
use self::match_char::{match_in_value, match_out, match_in_string_escape};

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

impl <I, JSON> TryFrom<State<I>> for JSON
    where I: TryInto<JSON, Err=TokenizeError>,
          JSON: MakeJSON,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>,
          <<JSON as MakeJSON>::Number as FromStr>::Err: Error + 'static{
    type Err = TokenizeError;
    fn try_from(s: State<I>) -> Result<JSON, Self::Err> {
        match s {
            State::Out(i) => i.try_into(),
            State::Error(msg) => Err(msg),
            _ => Err("Unmatched quotes".into())
        }
    }
}
