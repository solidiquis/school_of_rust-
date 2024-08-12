use std::fmt::{self, Display, Debug};

#[derive(Debug)]
struct Foo;

impl Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "I am Foo")
    }
}

// Static lifetime e.g. `&'static Foo`
// vs. 'static as a trait bound e.g.  `T: 'static`
//
// 'static as a trait bound accepts static references or OWNED types
//
// 'static as a trait bound is concerned with one thing: Can I hold the data indefinitely. In other
// words, can I hold the data for as long as I want and it will be valid.
fn print_foo<T>(val: T)
where
    T: 'static + Display + Debug
{
    println!("{val}");
    println!("{val:?}");
}

fn main() {
    let foo = Box::new(Foo{});
    let foo_static_ref: &'static Foo = Box::leak(foo);
    print_foo(foo_static_ref);

    let foo_stack = Foo{};
    print_foo(foo_stack);
}
