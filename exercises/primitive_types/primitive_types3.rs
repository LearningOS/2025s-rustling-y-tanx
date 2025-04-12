// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    // 数组类型
    // 对于包含非Copy类型的数组，需要注意所有权的转移
    // 数组初始化，需要提供初始化值和大小，必须提供初始化的值
    let a = [0; 1000];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
