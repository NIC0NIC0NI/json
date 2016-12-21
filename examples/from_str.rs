extern crate json;

use json::JSON;

fn main() {
    let json_str = stringify!(
        {
            "name" : "Element",
            "items" : 
            [
                1, 2, 3.5, false, true, 
                {
                    "something" : null
                }
            ],
            "empty" : []
        }
    );
    if let Ok(parsed) = json_str.parse::<JSON>(){
        println!("{:?}", parsed);
        if let Some(nvp) = parsed.as_map() {
            for (name, value) in nvp {
                println!("{} : {}", name, value);
            }
        }
    }
}
