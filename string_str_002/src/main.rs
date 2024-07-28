#![allow(unused_variables)]

fn main() {
    // Literal string.. embedded in binary image
    let literal: &str = "hello world";

    // String.. heap-allocated.
    let string: String = String::from("hello world");

    // Reference to a String
    let string_ref: &String = &string;

    // Reference to a string coerced into a &str
    let string_ref_as_str: &str = &string;
}
