// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();    // 字符串字面量是&str类型，就是String的引用

    get_char(data.clone());

    string_uppercase(data); // 发生了所有权的转移
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data.to_uppercase();

    println!("{}", data);
}
