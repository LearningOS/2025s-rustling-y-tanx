// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

// 模块的可见性与模块的作用域和模块声明方式有关
// 模块可以位于同一个文件中，或者分布在多个文件中，Rust 允许在同一个文件中访问私有模块内容，但 外部文件 中的代码无法访问到这些私有模块内容。
// 如果通不过模块的可见性检查，那么即使模块内部的函数或字段是Pub的也没法检查
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();    // Rust中模块、方法、字段的可见性类似于洋葱剥皮，一层一层地可见
}
