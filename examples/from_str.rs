extern crate json;

use json::JSON;

fn main() {
    let json_str = stringify!(
        {
            "name" : "Element",
            "items" : 
            [
                1, 2, 3, false, true, 
                {
                    "something" : null
                }
            ],
            "empty" : []
        }
    );
    if let Ok(parsed) = json_str.parse::<JSON>(){
        if let Some(nvp) = parsed.as_map() {
            for (name, _) in nvp {
                println!("{}", name);
            }
        }
    }
}
