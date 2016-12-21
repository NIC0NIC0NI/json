# JSON
A simple JSON parser in Rust. Practice purpose.

## Use
### Example
```
    #[macro_use(json_object)]
    extern crate json;

    use json::JSON;

    // construct JSON object directly.
    let json_obj = json_object!(
        {name: "Element", items: [1, 2, 3, false, true, {something: null}]}
    );    

    // JSON string
    let json_str = stringify!(
        {"name": "Element", "items": [1, 2, 3, false, true, {"something": null}]}
    );    
    
    match json_str.parse::<JSON>(){     // parse string to json object
        Ok(parsed) => assert_eq!(parsed, json_obj),
        Err(msg) => panic!("{}:\n{}", msg, json_str)
    }

```
### Documentation
Use `cargo doc` to generate the documents.

## implementation Details
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

## Testing Details

The two-phase processing is performed with pipelining, without storage of internal results. i.e. no `Vec<JSONToken>` appears. 

However, in testing `Vec<JSONToken>` should be constructed. This is organized by the `TokenConsumer` trait. While tokenizing, whenever a new token is generated, it is passed to `TokenConsumer::consume`. This trait is implemented by both the parser object with the parsing phase and `Vec<JSONToken>` with `push`.

## Limitations

It does check the syntax and will return error if it finds something wrong, but the error message is not so comprehensible. 

I used `std::collcetion::HashMap<String, JSON>` to represent key-value pairs without preserving the order. Replacing the definition of `json_object::JSONObject` with an ordered map should work. Unfortunately, type adaptors are not yet implemented. 

## References
### Specification
[ECMA-404.pdf](http://www.ecma-international.org/publications/files/ECMA-ST/ECMA-404.pdf)
### JavaScript
The parsing result is compared with JavaScript
```
    JSON.parse()
```
