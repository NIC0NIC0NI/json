#[macro_use(benchmark_group,benchmark_main)]
extern crate bencher;
extern crate json;
use bencher::Bencher;

use json::JSON;
use std::str::FromStr;

benchmark_group!(benches, basic, dence_tokens);
benchmark_main!(benches);

fn basic(b: &mut Bencher) {
    let json_str = stringify!(
        {
            "class" : "Element",
            "tag" : "a",
            "attributes" : [
                {
                    "class" : "Attribute",
                    "key" : "href",
                    "value" : "/publish/newthu/newthu_cnt/faculties/index.html"
                }
            ],
            "children" : [
                {
                    "class" : "TextNode",
                    "text" : "JSON",
                    "attributes" : [],
                    "children" : []
                }
            ]
        }
    );
    b.iter(|| {
        JSON::from_str(json_str)
    })
}

// to test extreme case that number(tokens)/number(characters) is high
fn dence_tokens(b: &mut Bencher) {
    let s = stringify!(
        {
            "a" : "b",
            "c" : "d",
            "e" :
            [
                {
                    "x" : 1.02,
                    "k" : false,
                    "v" : null
                }
            ],
            "c" : [
                {
                    "c" : "t",
                    "t" : "j",
                    "a" : [
                        1,2,3,4,5,6,6,7,8,8,3,3,3,2,3,4,2,5,6,7,
                        {
                            "a" : [
                                2,{
                                    "f" : 3,
                                    "g" : "x"
                                }
                            ]
                        }
                    ],
                    "h" : [
                        {
                            "a" : 3
                        },
                        {
                            "w" : false
                        },
                    ]
                }
            ]
        }
    );
    // remove white spaces 
    let json_str = &s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    b.iter(|| {
        JSON::from_str(json_str)
    })
}
