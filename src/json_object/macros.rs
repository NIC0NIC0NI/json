#[macro_export]
macro_rules! json_object {
    ( $T:ty : {}) => {{
        let object = <$T as $crate::MakeJSON>::Object::new();
        <$T as $crate::MakeJSON>::make_object(object)
    }};
    ( $T:ty : {$($name:ident : $value:tt),+}) => {{
        let mut object = <$T as $crate::MakeJSON>::Object::new();
        $(
            <<$T as $crate::MakeJSON>::Object as $crate::JSONObject>::add(
                &mut object, 
                stringify!($name).to_string(),
                json_object!($T: $value)
            );
        )*
        <$T as $crate::MakeJSON>::make_object(object)
    }};
    ( $T:ty : []) => {{
        let array = <$T as $crate::MakeJSON>::Array::new();
        <$T as $crate::MakeJSON>::make_array(array)
    }};
    ( $T:ty : [$($value:tt),+]) => {{
        let mut array = <$T as $crate::MakeJSON>::Array::new();
        $(
            <<$T as $crate::MakeJSON>::Array as $crate::JSONArray>::add(
                &mut array, 
                json_object!($T: $value)
            );
        )*
        <$T as $crate::MakeJSON>::make_array(array)
    }};
    ( $T:ty : null) => {
        <$T as $crate::MakeJSON>::make_null()
    };
    ( $T:ty : $x:expr) => {
       $crate::FromPremitive::from_premitive($x)
    };
}

#[macro_export]
macro_rules! json_default {
    ($x:tt) => {
        json_object!($crate::DefaultJSON: $x)
    };
}

#[macro_export]
macro_rules! json_preserving {
    ($x:tt) => {
        json_object!($crate::PreservingJSON: $x)
    };
}
