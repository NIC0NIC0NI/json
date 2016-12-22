#[macro_use(benchmark_group,benchmark_main)]
extern crate bencher;
extern crate json;
use bencher::Bencher;

use json::DefaultJSON as JSON;

benchmark_group!(benches, basic, dence_tokens, numbers);
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
                    "attributes" : [123],
                    "children" : [null, false]
                }
            ]
        }
    );
    b.iter(|| {
        json_str.parse::<JSON>()
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
        json_str.parse::<JSON>()
    })
}

fn numbers(b: &mut Bencher) {
    let json_str = stringify!(
        [1,2,3,123.456,123.4231,3214,421,10000000000000000000,20000000000000000000]
    );
    b.iter(|| {
        json_str.parse::<JSON>()
    })
}