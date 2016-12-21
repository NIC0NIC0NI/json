# JSON
A simple JSON parser in Rust, together with utilities, like a macro that implements JSON literal.

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
              +--------------------------------------------+
              |                                            |
              |                                            v
char input  --+-->  internal state of tokenizer ---->  token output

token input  -------------->  internal state of parser
```
After all characters are processed, we check the internal state of the parser. If it is in an expected state, then we unwrap it to get the result, else make an error message.

## Testing Details

The two-phase processing is performed with pipelining, without storage of internal results. i.e. in tokenizing phase, whenever a new token is generated, it is passed to the parsing phase, without constructing `Vec<JSONToken>`.

However, to test the correctness of the tokenizing, `Vec<JSONToken>` should be constructed and compared with the right answer. This is organized by the `TokenConsumer` trait. While tokenizing, whenever a new token is generated, it is passed to `TokenConsumer::consume`. This trait is implemented by both the parser object and `Vec<JSONToken>` with `push`.

## Issues

### Comprehensible Errors
It does check the syntax and will return error if it finds something wrong. But the error object contains nonthing but an error message, and the message is not so comprehensible. 

### More Generic
`std::collcetion::HashMap<String, JSON>` is used to represent key-value pairs without preserving the order. It should work by replacing the definition of `json_object::JSONObject` with an ordered map. Unfortunately, type adaptors are not yet implemented. 

What's more, even the data type to represent numbers and strings are not customizable.

### Stable Rust
Stable version of Rust is used, therefore even basic traits like `TryFrom` and `TryInto` are defined by myself. These will be removed once standard library is stabilized.

## References
### Specification
[ECMA-404.pdf](http://www.ecma-international.org/publications/files/ECMA-ST/ECMA-404.pdf)
### JavaScript
The parsing result is compared with JavaScript
```
    JSON.parse()
```
