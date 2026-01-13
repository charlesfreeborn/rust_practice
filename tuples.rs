fn main() {
    let tup: (i32, f64, u8, &str) = (100, 1.1, 1, "Do Hard Things!");

    // Destructure the tuple and bind the second element to x
    let (_, x, ..) = tup;
    println!("The value of x is: {}", x);

    // eleement at specific index can be accessed using dot notation
    let first_item = tup.0;
    let second_item = tup.1;
    let third_item = tup.2;
    let fourth_item = tup.3;
    println!(
        "The value of the first item is: {}, second item is: {}, third item is: {}, and fourth item is: {}",
        first_item, second_item, third_item, fourth_item
    );

    let s = String::from("Hello, Charles.");

    let (s1, len) = calculate_length(s);
    println!("The lenght of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
