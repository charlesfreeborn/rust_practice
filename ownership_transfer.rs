struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
}

fn main() {
    let mut foo = Foo {x: 42};
    let f = &mut foo;

    f.x = 13;

    println!("{}", foo.x);

    foo.x = 7;
    do_something(foo);
}