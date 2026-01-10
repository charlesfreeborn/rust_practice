use std::io;

fn main() {
    // create a mutable string variable to store user input
    let mut input: String = String::new();
    println!("Please enter your name:");
    // Read user input and store it in the input variable
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    // print the user-input string
    println!("Your name is: {}", input);
}
