fn main() {
    let a: &str = "hello, hackquest.";
    println!("{} go go go", a);

    // get hello by index
    let b = &a[..5];
    println!("{}", b);

    let s1: String = String::from("hello,world!");
    let s2: &str = &s1[..5];
    println!("{}", s2);

    let s3: &str = s1.as_str();

    let s4: String = "hello,world".to_string();
    let s5: String = String::from("hello.world");
}
