// move_semantics3.rs
//
// Make me compile without adding new lines-- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);  

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// 函数中能这么写本质上是因为发生了所有权的转移，即let mut vec1 = vec0
// 正常来说参数的可变性和声明中的可变性应该保持一致
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
