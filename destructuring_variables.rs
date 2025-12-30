struct Ferris {
    e: i32,
    f: String,
}

fn main() {
    let (a, b, c, d, e, f);

    // tuple destructure
    (a, b) = (1, 2);

    // array destructure ... means ignore multiple elements. _ means ignore the element at the roeesponding index position (1)
    [c, .., d, _] = [1, 2, 3, 4, 5];
    // struct destructure
    Ferris { e, f } = Ferris {
        e: 5,
        f: "rust".to_string(),
    };

    assert_eq!((1, 2, 1, 4, 5, "rust".to_string()), (a, b, c, d, e, f));

    println!("{}, {}, {}", a, c, d);
}
