mod json;

#[derive(PartialEq, Debug, Clone)]
pub enum PreservingJSON {
    Bool(bool),
    Number(String),  // to avoid overflow, just left strings
    String(String),
    Array(Vec<PreservingJSON>),
    Object(Vec<(String, PreservingJSON)>), // to preserve order
    Null
}


