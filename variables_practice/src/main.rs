fn main() {
    // mutable variables
    let mut x = 5;
    println!("X is originally assigned {}", x);

    x = 10;
    println!("x is now {}", x);

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(
        "Three hours in seconds is a fixed constant of {} seconds.",
        THREE_HOURS_IN_SECONDS
    )

    // shadowing example
}
