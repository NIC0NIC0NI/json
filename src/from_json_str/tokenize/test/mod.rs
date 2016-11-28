
use super::super::TokenConsumer;
use super::super::Tokenizer;
use super::super::JSONToken;
use super::super::Error;
use super::State;



impl TokenConsumer for Vec<JSONToken> {
    fn new() -> Self {
        Vec::new()
    }
    fn consume(mut self, token: JSONToken) -> Self {
        self.push(token);
        self
    }
}

fn tokenize_str<TC:TokenConsumer>(json_str: &str) -> Result<TC,Error> {
    let init:State<TC> = State::new();
    let result = json_str.chars().fold(init, State::tokenize).tokenize(' ');
    match result {
        State::Out (consumer) => Ok(consumer),
        State::Error (error) => Err(error),
        _ => Err("Unbalanced quotes ".to_string())
    }
}

#[test]
fn test(){
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
    let json_str = stringify!(
        {
            "class" : "Element",
            "tag" : "a",
            "attributes" :
            [
                {
                    "class" : "Attribute",
                    "key" : "href",
                    "value" : "/publish/newthu/newthu_cnt/faculties/index.html"
                }
            ],
            "children" :
            [
                {
                    "class" : "TextNode",
                    "text" : "JSON",
                    "attributes" : [],
                    "children" : []
                }
            ]
        }
    );
    match tokenize_str::<Vec<JSONToken>>(json_str) {
        Ok(tokenized) => assert_eq!(tokenized, tokens),
        Err(msg) => panic!("{}:\n{}", msg, json_str)
    }
}
