rust_option_size_experiment
===========================

[core::option - Rust](https://doc.rust-lang.org/stable/core/option/index.html)

> This usage of Option to create safe nullable pointers is so common that Rust does special optimizations to make the representation of Option<Box<T>> a single pointer. Optional pointers in Rust are stored as efficiently as any other pointer type.

## Run an experiment

```
$ cargo run
...(snip)...
Box<i32> size=8
Option<Box<i32>> size=8
*mut libc::c_char size=8
Option<*mut libc::c_char> size=16
$ rustc --version
rustc 1.3.0-nightly (e4e93196e 2015-07-14)
$ uname -s
Darwin
$ uname -m
x86_64
```
