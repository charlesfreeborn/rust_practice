fn main() {
    // u32-bit unsigned integer maximum value
    let a: u32 = 4_294_967_295;
    let b: u32 = 1;

    // output result
    println!("Original: {}", a);
    println!("Add 1: {}", b);

    // using wrapping * methods tis handled according to the two's complement loop overflow relues in all modes, if a value exceeds the maximum value it restarts counting from 0.
    let result_wrapping = a.wrapping_add(b);

    // result_wrapping 0
    println!("Wrapping Result: {}", result_wrapping);

    // saturating_* returns the maximum value of the corresponding type when the integer overflows and retrunds the minimum value if it undeflows
    let result_saturating = a.saturating_add(b);

    println!("Saturating Result: {}", result_saturating);

    // the return value of checked_* is Option, which is None when it overflows, otherwise it is Some (result). You can check whether the operation caused an overflow

    let result_checked = a.checked_add(b);
    // return None
    match result_checked {
        Some(result) => println!("Checked Result: {}", result),
        None => println!("Checked Result: Overflow!"),
    }

    let (result_overflowing, overflowed) = a.overflowing_add(b);
    if overflowed {
        println!("Overflowing Result: Overflow!");
    } else {
        println!("Overflowing Result: {}", result_overflowing);
    }
}
