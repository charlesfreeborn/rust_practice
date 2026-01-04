fn main() {
    // default type i32
    let sum = 1 + 2;

    // default type f64
    let difference = 1.1 - 2.2;

    // default type i32
    let mul = 2 * 50;

    // default type f64
    let div = 1.1 / 2.2;

    // default type i32
    let remainder = 11 % 5;

    println!(
        "sum: {}, difference: {}, mul: {}, div: {}, remainder: {}",
        sum, difference, mul, div, remainder
    );

    // compipler will deduce the type
    let ferris = 10;

    // explicit define the data type here
    let ferris_one: i32 = 10;

    // type annotation through type suffix: 10 is i32 type
    let ferris_two = 10i32;

    // Only operations of same type can be performed.
    let addition = ferris + ferris_one + ferris_two;
    println!(
        "{} + {} + {} = {}",
        ferris, ferris_one, ferris_two, addition
    );

    // define an f32 array, in which 1.0 will automaitcally be deduced as the f32 type
    let arr = [1.0, 1.1f32, 1.2_f32];

    // print the first value arr[0] in the array, and use {:.2} to control the decimal place to 2 digits
    println!("{:.2}", arr[0]);
}
