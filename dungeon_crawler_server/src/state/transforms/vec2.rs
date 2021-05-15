use std::ops::{Add, Mul, Sub};

use simple_serializer::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vec2(pub i32, pub i32);
impl Vec2 {
    pub fn from_tuple(tuple: (i32, i32)) -> Self {
        Vec2(tuple.0, tuple.1)
    }
    pub fn distance(&self, other: Vec2) -> f32 {
        f32::sqrt(((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2)) as f32)
    }
}
impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}
impl Serialize for Vec2 {
    type SerializeTo = String;

    fn serialize(&self) -> Self::SerializeTo {
        format!("{}::{}", self.0, self.1)
    }
}
