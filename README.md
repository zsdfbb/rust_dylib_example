# rust_dylib_example

Rust dynamic library test


## Introduction

- **myclib** is a dynamic library written in C language.
- **mylib** is a dynamic library written in Rust language. And it compiled by ***crate-type = ["dylib"]***.
- **myrclib** is a dynamic library written in Rust language. And it compiled by ***crate-type = ["cdylib"]***.
- **myelf** will link **mylib**, **myclib** **myrclib** dynamic libraries.

## Run

```bash
make run
```

## Question

### Q1: libstd-8df6be531efb3fd0.so is not libmylib.so

```text
$ make info_link
readelf -d myelf/target/release/myelf

Dynamic section at offset 0x2db8 contains 26 entries:
  Tag        Type                         Name/Value
 0x0000000000000001 (NEEDED)             Shared library: [libmyclib.so]
 0x0000000000000001 (NEEDED)             Shared library: [libstd-8df6be531efb3fd0.so]
 0x0000000000000001 (NEEDED)             Shared library: [libc.so.6]
 ```