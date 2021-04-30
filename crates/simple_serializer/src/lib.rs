pub trait Serialize {
    type SerializeTo;
    fn serialize(&self) -> Self::SerializeTo;
}

pub trait Deserialize {
    type DeserializeTo;
    fn deserialize(from: &str) -> Self::DeserializeTo;
}