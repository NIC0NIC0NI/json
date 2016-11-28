# JSON
A simple JSON parser in Rust. Practice purpose.

## Use
tests/basic.rs is an example
```
    #[macro_use(json_object)]
    extern crate json;

    use json::JSON;
    use std::str::FromStr;

    let json_obj = json_object!(
        {name: "Element", items: [1, 2, 3, false, true, {something: null}]}
    );    // construct JSON object directly

    let json_str = stringify!(
        {"name": "Element", "items": [1, 2, 3, false, true, {"something": null}]}
    );    // JSON string
    
    match JSON::from_str(json_str){     // parse string to json object
        Ok(parsed) => assert_eq!(parsed, json_obj),
        Err(msg) => panic!("{}:\n{}", msg, json_str)
    }

```

## Parsing Details
The original character stream is preprocessed by "tokenize"
```
                   input              output                  
character stream --------> tokenize ---------> token stream
                                                    |
                   output              input        |
   JSON Tree     <--------  parse   <---------------+
```
Tokeinization and parsing are based on finite state machine, with states represented by `enum`.

```
              +->  internal state of tokenizer ------------+
              |                                            |
              |                                            v
char input  --+------------------------------------>  token output

token input  -------------->  internal state of parser
```
After all characters are processed, we check the internal state of the parser. If it is in an expected state, then we unwrap it to get the result, else make an error message.

## Limitations

It does check the syntax and will return error if find it wrong, but the error message is not so comprehensible. 
Maybe a counter should be added to record the error location.

I used `std::collcetion::HashMap<String, JSON>` to represent key-value pairs without preserving the order. Replacing the definition of `$crate::json_object::NameValuePair` with an ordered map should work.

## References
### Specification
[ECMA-404.pdf](http://www.ecma-international.org/publications/files/ECMA-ST/ECMA-404.pdf)
### JavaScript
The parsing result is compared with JavaScript
```
    JSON.parse()
```
