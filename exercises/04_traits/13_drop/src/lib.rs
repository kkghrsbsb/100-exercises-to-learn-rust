// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

// 答案的defused是扩散的意思，让我们知道爆炸有没有扩散(bool),真的自己写不明白
pub struct DropBomb {
    defused: bool,
}

impl DropBomb {
    pub fn new() -> Self {
        DropBomb { defused: false }
    }

    pub fn defuse(&mut self) {
        self.defused = true;
    }
}

// Drop 可以随便实现，不需要真的管理资源 (对类型实现Drop -> 编译器认为你需要特别处理你的生命周期，且此类型不能显式是Copy了)
impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.defused {
            panic!("Boom!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
