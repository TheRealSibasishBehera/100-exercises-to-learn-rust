// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct SaturatingU16 {
    value: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {
            value: value as u16,
        }
    }
}
impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 {
            value: *value as u16,
        }
    }
}
impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 { value: *value }
    }
}

// pub trait Add<Rhs = Self> {
//     /// The resulting type after applying the `+` operator.
//     #[stable(feature = "rust1", since = "1.0.0")]
//     type Output;
//
//     /// Performs the `+` operation.
//     ///
//     /// # Example
//     ///
//     /// ```
//     /// assert_eq!(12 + 1, 13);
//     /// ```
//     #[must_use = "this returns the result of the operation, without modifying the original"]
//     #[rustc_diagnostic_item = "add"]
//     #[stable(feature = "rust1", since = "1.0.0")]
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

impl Add<u16> for SaturatingU16 {
    type Output = u16;
    fn add(self, rhs: u16) -> u16 {
        self.value.saturating_add(rhs)
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &SaturatingU16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

impl Add for SaturatingU16 {
    type Output = u16;
    fn add(self, rhs: Self) -> u16 {
        self.value.saturating_add(rhs.value)
    }
}
