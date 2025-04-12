// enums1.rs
//
// No hints this time! ;)

// Rust中枚举是一个强大的类型，枚举成员可以有自己关联的字段，类似结构体，而且像结构体一样，可以为枚举实现方法。Option和Result实际上就是枚举类型
// 相较于结构体，枚举成员的类型都是同一个=>枚举本身，因此在函数中可以用统一的声明来操作它们

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
