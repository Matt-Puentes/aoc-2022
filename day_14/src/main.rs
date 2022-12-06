mod parse_args;
use parse_args::{parse_args, Test};

pub fn pt_1(str_input: &str) {
    println!("Part 1 result: {}", str_input.len().try_into().unwrap_or(0))
}

pub fn pt_2(str_input: &str) {
    println!("Part 2 result: {}", str_input.len().try_into().unwrap_or(0))
}

pub fn main() {
    println!("Running day 14");
    match parse_args() {
        Test::One(text) => pt_1(text),
        Test::Two(text) => pt_2(text),
        Test::Both(text) => {
            pt_1(text);
            pt_2(text);
        }
    }
}
