use ::std::char::from_u32;
use ::std::str::FromStr;

use super::State;
use super::super::JSONToken;
use super::super::TokenConsumer;
use super::super::ParseError as Error;

enum Escape {
    Character(char),
    Unicode,
    Error
}

fn is_structure(c: char) -> bool {
    c == ',' || c == ':' || c == '{' || c == '}' || c == '[' || c == ']'
}
fn to_structure(c: char) -> Option<JSONToken> {
    match c {
        ',' => Some(JSONToken::Comma),
        ':' => Some(JSONToken::Colon),
        '{' => Some(JSONToken::LeftBrace),
        '}' => Some(JSONToken::RightBrace),
        '[' => Some(JSONToken::LeftBracket),
        ']' => Some(JSONToken::RightBracket),
        _ => None
    }
}

pub fn match_in_string<TC:TokenConsumer>(c: char, consumer: TC, mut word: String) -> State<TC>{
    match c {
        '\"' => {
            State::Out(consumer.consume(JSONToken::StringToken(word)))
        },
        '\\' => State::InStringEscape(consumer, word),
        cx if cx.is_control() => State::Error(format!(
            "Unexpected control character, use '{}' instead", 
            c.escape_default().collect::<String>()
        ).into()),
        x => {
            word.push(x);
            State::InString(consumer, word)
        }
    }
}

pub fn match_in_string_escape<TC:TokenConsumer>(c: char, consumer: TC, mut word: String) -> State<TC> {
    let e = match c {
        't' => Escape::Character('\t'),
        'f' => Escape::Character('\u{000C}'),
        'b' => Escape::Character('\u{0008}'),
        'n' => Escape::Character('\n'),
        'r' => Escape::Character('\r'),
        '\\' => Escape::Character('\\'),
        '/' => Escape::Character('/'),
        '\"' => Escape::Character('\"'),
        'u' => Escape::Unicode,
        _ => Escape::Error
    };
    match e {
        Escape::Character(cc) => {
            word.push(cc);
            State::InString(consumer, word)
        },
        Escape::Error => State::Error("Invalid escape character".into()),
        Escape::Unicode => State::InStringEscapeUnicode(consumer, word, String::with_capacity(4))
    }
}

pub fn match_in_string_escape_unicode<TC:TokenConsumer>(c: char, consumer: TC, 
        mut word: String, mut escape: String) -> State<TC> {
    if escape.len() == 4{
        if let Ok(codepoint) = u32::from_str_radix(&escape, 16u32) {
            if let Some(character) = from_u32(codepoint) {
                word.push(character);
                match_in_string(c, consumer, word)
            } else {
                State::Error(format!("Unexpected unicode escape: \\u{}", escape).into())
            }
        } else {
            State::Error(format!("Unexpected unicode escape: \\u{}", escape).into())
        }
    } else if c.is_digit(16) {
        escape.push(c);
        State::InStringEscapeUnicode(consumer, word, escape)
    } else {
        State::Error(format!("Unexpected unicode escape: \\u{}", escape).into())
    }
}

fn parse_value(s: &str) -> Result<JSONToken, Error> {
    match s {
        "true" => Ok(JSONToken::BoolToken(true)),
        "false" => Ok(JSONToken::BoolToken(false)),
        "null" => Ok(JSONToken::NullToken),
        _ => {
            if let Ok(i) = i64::from_str(s) {
                Ok(JSONToken::IntToken(i))
            } else if let Ok(f) = f64::from_str(s) {
                Ok(JSONToken::FloatToken(f))
            } else {
                Err(format!("Invalid value literal: {}" ,s).into())
            }
        }
    }
}

pub fn match_in_value<TC:TokenConsumer>(c: char, consumer: TC, mut word: String) -> State<TC> {
    match c {
        cx if cx.is_whitespace() => {
            match parse_value(&word) {
                Ok(v) => {
                    State::Out(consumer.consume(v))
                },
                Err(msg) => State::Error(msg)
            }
        },
        cx if is_structure(cx) => {
            match parse_value(&word) {
                Ok(v) => {
                    State::Out(consumer.consume(v).consume(to_structure(cx).unwrap()))
                },
                Err(msg) => State::Error(msg)
            }
        }
        cx => {
            word.push(cx);
            State::InValue(consumer, word)
        }
    }
}

pub fn match_out<TC:TokenConsumer>(c: char, consumer: TC) -> State<TC> {
    match c {
        '\"' => State::InString(consumer, String::new()),
        cx if is_structure(cx) => {
            State::Out(consumer.consume(to_structure(cx).unwrap()))
        },
        cx if cx.is_whitespace() => {
            State::Out(consumer)
        },
        cx => {
            let mut word = String::new();
            word.push(cx);
            State::InValue(consumer, word)
        }
    }
}
