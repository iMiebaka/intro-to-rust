use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
mod basic;
mod utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sum_number() {
        let sum = basic::let_sum_number(12, 5);
        assert_eq!(sum, 17);
    }
}

fn main() {
    let stdout = stdout();
    let message = String::from("Welcome Miebaka");
    let width = message.chars().count();
    let va:i32 = basic::let_sum_number(1, 2);
    println!("{} is the sum", va);
    utils::metal::display_metal_info();
    utils::solid::sold_guy();

    println!("The number is: {}", width);
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
