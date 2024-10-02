//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // 在 tests7 中，我们设置一个名为 TEST_FOO 的环境变量
    // 将当前时间戳作为该变量的值，并通过 `println!` 输出到标准输出。
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // 用当前时间戳作为环境变量 TEST_FOO 的值
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 在 tests8 中，我们启用 "pass" 特性标志来使测试提前返回
    // 通过 `println!` 输出到标准输出。
    println!("cargo:rustc-cfg=feature = "pass",");
}
