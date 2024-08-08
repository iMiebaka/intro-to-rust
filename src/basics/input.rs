use core::num;
use std::io;

fn basic_input(){
    let stdin = io::stdin();
    let mut result = String::new();
    stdin.read_line(&mut result).expect("Something went wrong");
    println!("{}", result.len());

    let number: i32 = match result.trim().parse(){
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to parse number: {}", e);
            return;
        }
    };

    println!("{}", number * 2);


    let numbers: Vec<i32> = result
    .trim()  // Remove leading and trailing whitespace
    .split_whitespace()  // Split by whitespace
    .map(|s| s.parse()   // Parse each substring into an integer
        .expect("Failed to parse number"))  // Handle parse errors
    .collect();  // Collect into a vector


    println!("{:?}", numbers)
    // for line in stdin.lock().lines() {
    //     println!("{}", line.unwrap());
    // }
}