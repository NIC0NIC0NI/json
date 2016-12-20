
/// Similar to `json_object` macro.
/// Used internally for unit tests.
macro_rules! json_tokens {
    ( [$($item:tt),+] ) => {{
        let mut tokens = vec![$crate::from_json_str::JSONToken::LeftBracket];
        $(
            tokens.extend(json_tokens!($item));
            tokens.push($crate::from_json_str::JSONToken::Comma);
        )*
        tokens.pop();
        tokens.push($crate::from_json_str::JSONToken::RightBracket);
        tokens
    }};
    ( [] ) => {
        vec![$crate::from_json_str::JSONToken::LeftBracket, $crate::from_json_str::JSONToken::RightBracket]
    };


    ( {$($name:ident : $value:tt),+} ) => {{
        let mut tokens = vec![$crate::from_json_str::JSONToken::LeftBrace];
        $(
            tokens.push($crate::from_json_str::JSONToken::StringToken(stringify!($name).to_string()));
            tokens.push($crate::from_json_str::JSONToken::Colon);
            tokens.extend(json_tokens!($value));
            tokens.push($crate::from_json_str::JSONToken::Comma);
        )*
        tokens.pop();
        tokens.push($crate::from_json_str::JSONToken::RightBrace);
        tokens
    }};
    ( {} ) => {
        vec![$crate::from_json_str::JSONToken::LeftBrace, $crate::from_json_str::JSONToken::RightBrace]
    };


    (null) => {
        vec![$crate::from_json_str::JSONToken::NullToken]
    };
    ($x:expr) => {
        vec![$crate::from_json_str::json_token::TokenValue::to_token($x)]
    };
}