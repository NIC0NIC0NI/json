/// Construct JSON object using JSON literal
#[macro_export]
macro_rules! json_object {
    ( $T:ty : {}) => {{
        let object = <$T as $crate::MakeJSON>::Object::new();
        <$T as $crate::MakeJSON>::make_object(object, &Vec::new())
    }};
    ( $T:ty : {$($name:ident : $value:tt),+}) => {{
        let mut object = <$T as $crate::MakeJSON>::Object::new();
        $(
            <<$T as $crate::MakeJSON>::Object as $crate::JSONObject>::add(
                &mut object, 
                stringify!($name).to_string(),
                json_object!($T: $value),
                &Vec::new()
            );
        )*
        <$T as $crate::MakeJSON>::make_object(object, &Vec::new())
    }};
    ( $T:ty : []) => {{
        let array = <$T as $crate::MakeJSON>::Array::new();
        <$T as $crate::MakeJSON>::make_array(array, &Vec::new())
    }};
    ( $T:ty : [$($value:tt),+]) => {{
        let mut array = <$T as $crate::MakeJSON>::Array::new();
        $(
            <<$T as $crate::MakeJSON>::Array as $crate::JSONArray>::add(
                &mut array, 
                json_object!($T: $value),
                &Vec::new()
            );
        )*
        <$T as $crate::MakeJSON>::make_array(array, &Vec::new())
    }};
    ( $T:ty : null) => {
        <$T as $crate::MakeJSON>::make_null(&Vec::new())
    };
    ( $T:ty : $x:expr) => {
       $crate::FromPremitive::from_premitive($x)
    };
}

/// Construct `DefaultJSON` using JSON literal
#[macro_export]
macro_rules! json_default {
    ($x:tt) => {
        json_object!($crate::DefaultJSON: $x)
    };
}

/// Construct `PreservingJSON` using JSON literal
#[macro_export]
macro_rules! json_preserving {
    ($x:tt) => {
        json_object!($crate::PreservingJSON: $x)
    };
}
