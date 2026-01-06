use std::thread;
use std::time::Duration;

// function takes 3 seconds
fn get_calculate_result() -> bool {
    // simulating complex calculations takes 3 seconds
    thread::sleep(Duration::from_secs(3));
    println!("Called this function");
    false
}

fn main() {
    // Print individual characters in various languages
    let thai_char = 'n';
    let korean_char = 'ㅂ';
    let traditional_chinese_char = '學';
    let indonesian_char = 'é';

    let _str = "話";
    println!("thai_char : {}", thai_char);
    println!("Korean: {}", korean_char);
    println!("Traditional Chinese: {}", traditional_chinese_char);
    println!("Indonesian: {}", indonesian_char);

    // Test how many characters are there in between '你' and '我'
    for i in '你'..='我' {
        print!("{}", i);
    }

    let f: bool = true;

    if f || get_calculate_result() {
        println!("Success!");
    }
}
