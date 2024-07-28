struct Foo {
    id: u8,
}

// Immutable borrow
fn borrow_and_print_id(obj: &Foo) {
    println!("{}", obj.id);
}

// Mutable borrow
fn mutable_borrow_assign_and_print(obj: &mut Foo) {
    obj.id += 1;
    println!("{}", obj.id);
}

// Mutable move
fn mutable_move_and_assign_and_print(mut obj: Foo) {
    obj.id += 1;
    println!("{}", obj.id);
}

// Immutable move
fn move_and_assign_and_print(obj: Foo) {
    println!("{}", obj.id);
}

fn main() {
    let mut obj = Foo { id: 8 };
    mutable_borrow_assign_and_print(&mut obj);
    mutable_move_and_assign_and_print(obj);
}
