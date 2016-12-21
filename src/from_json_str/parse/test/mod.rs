use super::State;
use super::super::{JSONToken,TokenConsumer,ParseError as Error};
use super::super::super::JSON;

fn parse_tokens(tokens: Vec<JSONToken>) -> Result<JSON,Error> {
    let init = State::new();
    let result = tokens.into_iter().fold(init, State::consume);
    match result {
        State::End(json) => Ok(json),
        State::Error(error) => Err(error),
        State::Begin => Err("Empty string".into()),
        State::ObjectBegin(_, _) | State::ObjectWithName(_, _, _) | 
            State::ObjectWithColon(_, _, _) | State::ObjectWithValue(_, _) |
                State::ObjectWithComma(_, _) => Err("Unmatched braces".into()),
        State::ArrayBegin(_, _) | State::ArrayWithValue(_, _) |
            State::ArrayWithComma(_, _) => Err("Unmatched brackets".into()),
    }
}

#[test]
fn test(){
    let object = json_object!(
        {
            class : "Element",
            tag : "a",
            attributes :
            [
                {
                    class : "Attribute",
                    key : "href",
                    value : "/publish/newthu/newthu_cnt/faculties/index.html"
                }
            ],
            children :
            [
                {
                    class : "TextNode",
                    text : "JSON",
                    attributes : [],
                    children : []
                }
            ]
        }
    );
    let tokens = json_tokens!(
        {
            class : "Element",
            tag : "a",
            attributes :
            [
                {
                    class : "Attribute",
                    key : "href",
                    value : "/publish/newthu/newthu_cnt/faculties/index.html"
                }
            ],
            children :
            [
                {
                    class : "TextNode",
                    text : "JSON",
                    attributes : [],
                    children : []
                }
            ]
        }
    );
    match parse_tokens(tokens) {
        Ok(parsed) => assert_eq!(parsed, object),
        Err(msg) => panic!("{}", msg)
    }
}