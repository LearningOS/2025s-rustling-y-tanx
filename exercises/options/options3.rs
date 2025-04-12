// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
    // 这发生了所有权的转移，所有权的转移会发生在模式匹配、函数传参
    // 这里，y内部Point变量的所有权转移到p上了，而p的生命周期结束了，因此这里会报错
    // 迭代、match、函数传参尽量都传入引用，这样内部的元素也是引用，不会出现所有权的转移
}
