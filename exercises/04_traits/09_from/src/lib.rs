// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

use std::convert::From;
impl From<i32> for WrappingU32 {
    fn from(value: i32) -> Self {
        Self {
            value: value as u32,
        }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
