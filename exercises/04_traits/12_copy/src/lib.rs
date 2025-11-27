// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

// 描述数据结构的trait可以用到派生宏
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

// 描述行为逻辑的trait，必须手动实现
impl std::ops::Add for WrappingU32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value.wrapping_add(rhs.value))
        // 还记得第一章的wrapping_add吗，我们设过全局cargo，overflow-checks = true，整数溢出会崩溃
        // 测试中传递了u32::MAX,告诉我们要用到它了
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
