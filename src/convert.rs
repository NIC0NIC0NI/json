

/// Will be removed once `std::convert::TryInto` is stabilized
pub trait TryInto<T>{
    type Err;
    fn try_into(self) -> Result<T, Self::Err>;
}
/// Will be removed once `std::convert::TryFrom` is stabilized
pub trait TryFrom<T>{
    type Err;
    fn try_from(t: T) -> Result<Self, Self::Err> where Self: Sized;
}

/// `std::iter::FromIterator` that may fail
pub trait TryFromIterator<Item>{
    type Err;
    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Err>
         where I: IntoIterator<Item=Item>, Self: Sized;
}

/// Automatically implemented if implemented `JSONArray`,`JSONObject` and `MakeJSON`
pub trait FromJSONStr {
    type Err;
    fn from_json_str(s: &str) -> Result<Self, Self::Err>
        where Self: Sized;
}

/// Implement `FromJSONStr<(Number)>`, `FromJSONStr<&str>`
/// and `FromJSONStr<bool>` to use `json_object` macro 
pub trait FromPremitive<P> {
    fn from_premitive(p: P) -> Self
        where Self: Sized;
}

impl <T,U> TryInto<T> for U
    where T: TryFrom<U>{
    type Err = <T as TryFrom<U>>::Err;
    fn try_into(self) -> Result<T, Self::Err>{
        <T as TryFrom<U>>::try_from(self)
    }
}

impl <T,U> TryFrom<Box<T>> for U 
    where T: TryInto<U> {
    type Err = <T as TryInto<U>>::Err;
    fn try_from(t: Box<T>) -> Result<Self, Self::Err>{
        (*t).try_into()
    }
}
