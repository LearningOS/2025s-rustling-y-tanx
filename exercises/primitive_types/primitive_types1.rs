// primitive_types1.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)
//
// Execute `rustlings hint primitive_types1` or use the `hint` watch subcommand
// for a hint.


fn main() {
    // Booleans (`bool`)

    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    // Rust中bool类型的变量只能有两个值：true和false，与C++不同，数值型变量不能作为布尔型变量来使用
    // 即Rust不能自动实现非bool类型到bool类型的转换，if之后必须跟着明确的bool类型的变量  
    let is_evening = false;// Finish the rest of this line like the example! Or make it be false!
    if is_evening {
        println!("Good evening!");
    }
}
