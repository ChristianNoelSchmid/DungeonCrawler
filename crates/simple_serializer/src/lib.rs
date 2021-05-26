///
/// A trait that requires that a particular struct
/// serializes to a generic type.
///
pub trait Serialize {
    type SerializeTo;
    fn serialize(&self) -> Self::SerializeTo;
}

///
/// A trait that requires that a particular struct
/// deserializes from an &str into a generic type.
///
pub trait Deserialize {
    type DeserializeTo;
    fn deserialize(from: &str) -> Self::DeserializeTo;
}
