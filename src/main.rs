extern crate libc;
use std::mem;

fn main() {
    println!("Box<i32> size={}", mem::size_of::<Box<i32>>());
    println!("Option<Box<i32>> size={}", mem::size_of::<Option<Box<i32>>>());

    println!("*mut libc::c_char size={}", mem::size_of::<*mut libc::c_char>());
    println!("Option<*mut libc::c_char> size={}", mem::size_of::<Option<*mut libc::c_char>>());
}
