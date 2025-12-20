// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;
// TODO：实现 `sum` 函数的多线程版本
//  使用 `spawn` 和 `join`。
//  给定一个整数向量，将其分成两半，并
//   在单独的线程中分别对每一半求和。

// 注意：我们无法测试函数的*实现方式*，
// 我们只能验证它是否产生了正确的结果。
// 你可以直接返回 `v.iter().sum()` 来通过此测试，
// 但这会违背练习的目的。

// 提示：你无法让生成的线程直接借用向量的切片。
// 你需要为原始向量的每一半分配新的向量。
// 我们将在下一个练习中看到为什么这是必要的。
pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    let (v1, v2) = v.split_at(mid);
    let v1 = v1.to_vec();
    let v2 = v2.to_vec();

    let handle1 = thread::spawn(move || v1.into_iter().sum::<i32>());
    let handle2 = thread::spawn(move || v2.into_iter().sum::<i32>());

    handle1.join().unwrap() + handle2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
