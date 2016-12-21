/// Construct JSON object use JSON syntax. Note that property names are not quoted.
/// # Examples
/// ```
/// #[macro_use]
/// extern crate json;
/// use json::JSON;
/// fn main(){
///     let json_obj:JSON = json_object!(
///         { first_property : "good", second_property : [1, 2, 3, false, null]}   
///     );
/// }
/// ```
#[macro_export]
macro_rules! json_object {
    ( [$($item:tt),+] ) => {{
        let mut vector = Vec::new();
        $(
            vector.push(json_object!($item));
        )*
        $crate::JSON::Array(vector)
    }};
    // specialized in order to get rid of "unused mutable" warning
    ( [] ) => {{
        $crate::JSON::Array(Vec::new())
    }};

    ( {$($name:ident : $value:tt),+} ) => {{
        let mut hash_map = ::std::collections::HashMap::new();
        $(
            hash_map.insert(stringify!($name).to_string(), json_object!($value));
        )*
        $crate::JSON::Object(hash_map)
    }};
    // get rid of warning
    ( {} ) => {{
        $crate::JSON::Object(::std::collections::HashMap::new())
    }};

    (null) => {
        $crate::JSON::Null
    };
    ($x:expr) => {
        $crate::JSON::from($x)
    };
}
