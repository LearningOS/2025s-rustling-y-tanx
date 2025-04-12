// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    // 元组，可以将一些变量打包在一起返回，尤其是不同类型的数据
    // 类似Python中的元组，它支持解构，可以匹配获得单个元素
    let cat = ("Furry McFurson", 3.5);
    let /* your pattern here */(name, age) = cat;

    println!("{} is {} years old.", name, age);
}
