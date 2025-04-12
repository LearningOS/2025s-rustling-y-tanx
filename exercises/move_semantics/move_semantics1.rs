// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.
// 这是所有权转移，Rust中一个需要防范和注意的地方，尽量避免写可能导致所有权转移的代码


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}   // 所有权发生了三次转移：从vec0 -> 参数vec，参数vec -> 变量vec，变量vec -> vec1
