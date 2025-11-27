pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
    let _size_of_str = size_of::<&str>();
    // 没有引用`&` 报错: 没有为 `str` 实现特征 `Sized` [E0277]
    // 所以str是没有实现Sized的
}
