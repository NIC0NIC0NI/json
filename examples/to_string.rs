#[macro_use(json_object)]
extern crate json;

fn main() {
    let json_obj = json_object!(
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