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
//
// TODO：定义一个新的 `SaturatingU16` 类型。
//  它应该存储一个 `u16` 值。
//  它应该提供从 `u16`、`u8`、`&u16` 和 `&u8` 到 `SaturatingU16` 的转换。
//  它应该支持右侧为 `SaturatingU16`、`u16`、`&u16` 和 `&SaturatingU16` 类型的加法运算。加法运算应在 `u16` 的最大值处达到饱和。
//  应该可以将其与另一个 `SaturatingU16` 或 `u16` 进行比较。
//  应该可以打印其调试表示。
//
// 测试位于 `tests` 文件夹中——请注意类型和方法的可见性。

use std::ops::Add;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct SaturatingU16 {
    num: u16,
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.num == *other
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self { num: value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self { num: value as u16 }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self { num: *value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        Self { num: *value as u16 }
    }
}

impl Add for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            num: self.num.saturating_add(rhs.num),
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: u16) -> Self::Output {
        Self {
            num: self.num.saturating_add(rhs),
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &u16) -> Self::Output {
        Self {
            num: self.num.saturating_add(*rhs),
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        Self {
            num: self.num.saturating_add((*rhs).num),
        }
    }
}
