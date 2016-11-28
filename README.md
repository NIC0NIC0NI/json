# JSON
A simple JSON parser in Rust. Practice purpose.

### Use
An example in tests/basic.rs
```
    use json::JSON;
    use std::str::FromStr;

    let json_obj = json_object!(
        {name : "Element", items : [1, 2, 3, false, true, {something : null}]}
    );    // construct JSON object directly

    let json_str = stringify!(
        {"name" : "Element", "items" : [1, 2, 3, false, true, {"something" : null}]}
    );    // JSON string
    
    match JSON::from_str(json_str){
        Ok(parsed) => assert_eq!(parsed, json_obj),
        Err(msg) => panic!("{}:\n{}", msg, json_str)
    }

```