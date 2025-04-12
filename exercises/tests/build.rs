//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // 可以给日志打时间戳、生成唯一ID、和后端API交互的时间同步字段
     
    // 设置环境变量TEST_FOO为时间戳 rustc-env=环境变量名=值
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // 启用pass feature：rustc-cfg=featur=feature名
    println!("cargo:rustc-cfg=feature=\"pass\"");
    println!("cargo:rerun-if-changed=exercises/tests/build.rs");
}
