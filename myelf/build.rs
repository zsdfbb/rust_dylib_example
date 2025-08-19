fn main() {
    println!("hello, build.");
    // 告诉编译器动态库搜索路径
    println!("cargo:rustc-link-search=native=../myclib");
    // 告诉编译器链接哪个库（去掉前缀 lib）
    println!("cargo:rustc-link-lib=myclib");

    println!("cargo:rustc-link-search=native=../myrclib/target/release/deps");
    println!("cargo:rustc-link-lib=myrclib");
}

