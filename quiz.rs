fn main() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // assert_eq!(x, __);

    assert_eq!(x, 3);

    // assert_eq!(y, __);

    assert_eq!(y, 2);
    println!("{}, {}", x, y);
}
