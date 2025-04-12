// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(3);
}

// Rust函数签名：fn 函数名(参数1: 类型, 参数2：类型) -> 返回类型 (可能还有泛型<T>，生命周期<'a>等)
fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
