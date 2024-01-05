use std::ops::Add;


/// Is always 42
pub struct Always42(u8);

impl Add for Always42 {
    type Output = Self;

    fn add(self, _: Self) -> Self::Output {
        Always42(42)
    }
}
