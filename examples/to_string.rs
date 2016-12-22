#[macro_use]
extern crate json;

fn main() {
    let json_obj = json_preserving!(
        {
            name : "Element",
            items : 
            [
                1, 2, 3, false, true, 
                {
                    something : null
                }
            ],
            empty : []
        }
    );
    let json_str = json_obj.to_string();
    println!("{}", json_str);
}