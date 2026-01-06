fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let v = {
        let mut x = 1;
        x + 2
    };

    assert_eq!(v, 3);

    let s = sum(1, 2);
    assert_eq!(s, 3);
}
