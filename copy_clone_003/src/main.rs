#[derive(Copy, Clone)]
struct Foo {
    a: u8,
    b: u8,
}

// Move
fn print_string(s: String) {
    println!("{s}");
}

// Move
fn print_integer(i: u8) {
    println!("{i}");
}

fn main() {
    // Clone
    let foo = "hello world".to_string(); // heap allocation
    print_string(foo.clone()); // heap allocation
    print_string(foo); // no new allocation

    // Copy
    let bar = 3; // stack allocation
    print_integer(bar); // stack allocation
    print_integer(bar); // stack allocation
}
