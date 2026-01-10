use std::mem::size_of_val;

fn main() {
    let x = String::from("Hello, hackquest.");
    // Borrow varibale x here
    let y = &x;

    println!("The memory address of x is {:p}", y);

    let mut s = String::from("hello, ");

    let p = &mut s;

    p.push_str(", hackquest.");
}
