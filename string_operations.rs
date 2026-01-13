fn main() {
    let mut s = String::from("hello ");

    // append a string: modify the optional string, not genertae a new string
    s.push_str("rust");
    println!("Append string push_str() -> {}", s);

    // Append characters
    s.push('!');
    println!("Append character push() -> {}", s);

    // Insert characters and modify the original string.
    s.insert(5, ',');
    println!("Insert chanacter insert() -> {}", s);

    // Insert string
    s.insert_str(6, " I like");
    println!("Insert string insert_str() -> {}", s);

    let str_old = String::from("I like rust, rust, rust!");
    let str_new = str_old.replace("rust", "RUST");
    println!(
        "The optinonal string length is: {}, memory address: {:p}",
        str_old.len(),
        &str_old
    );
    println!(
        "The new string length is: {}, memory address: {:p}",
        str_new.len(),
        &str_new
    );

    let mut string_pop = String::from("删除操作，rust 中文!");

    let p1 = string_pop.pop();
    println!("p1:{:?}", p1);

    let p2 = string_pop.pop();
    println!("p2:{:?}", p2);
    println!("string_pop:{:?}", string_pop);
}
