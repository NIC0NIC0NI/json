extern crate json;

use json::DefaultJSON as JSON;

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
    }
}
