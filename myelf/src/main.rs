use std::{iter::Sum, os::raw::{c_char, c_int}};

unsafe extern "C" {
    // 与 myclib.h 中的声明一致
    unsafe fn c_add(a: c_int, b: c_int) -> c_int;
}


fn main() {
    println!("Hello, world!");

    // test pure rust dylib
    let res = mylib::add(3, 4);
    println!("res: {}", res);

    // test pure c dylib
    let sum = unsafe { c_add(3, 4) };
    println!("res: {}", sum);

    // test rust c-abi dylib
}
