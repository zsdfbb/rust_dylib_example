use std::{os::raw::{c_int, c_longlong}};

// From clib
unsafe extern "C" {
    // 与 myclib.h 中的声明一致
    unsafe fn c_add(a: c_int, b: c_int) -> c_int;
}

// From rclib
unsafe extern "C" {
    // 与 myclib.h 中的声明一致
    unsafe fn rc_add(a: c_longlong, b: c_longlong) -> c_longlong;
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
    let sum = unsafe { rc_add(3, 4) };
    println!("res: {}", sum);
}
