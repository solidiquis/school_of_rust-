
struct Foo;

#[derive(Debug)]
struct Bar;

const ENV_TYPE: &'static str = "Hello";

fn leak_foo() -> &'static Foo {
    let boxed_foo = Box::new(Foo{});
    let foo: &'static Foo = Box::leak(boxed_foo);
    foo
}

fn main() {
    let foobar: &'static str = "foobar";

    let static_foo: &'static Foo = leak_foo();

    {
        let bar = Bar{};
        let bar_ref = &bar;
        println!("{bar_ref:?}");
    }


    println!("{ENV_TYPE}");
    println!("{foobar}");
}
