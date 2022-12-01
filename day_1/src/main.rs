mod parse_args;
use parse_args::{parse_args, Test};

fn pt_1(str_input: &str) {
    let res: usize = str_input
        .split("\n\n") // Split into blocks
        .map(|s| s.lines().map(|s| s.trim().parse::<usize>().unwrap()).sum())
        .max()
        .expect("List is empty");
    println!("Part 1 result: {}", res)
}

fn pt_2(str_input: &str) {
    let mut vec: Vec<usize> = str_input
        .split("\n\n") // Split into blocks
        .map(|s| s.lines().map(|s| s.trim().parse::<usize>().unwrap()).sum())
        .collect();
    vec.sort_unstable();
    vec.reverse();
    println!("Part 2 result: {}", vec[0] + vec[1] + vec[2])
}

fn main() {
    println!("Running day 1");
    match parse_args() {
        Test::One(text) => pt_1(text),
        Test::Two(text) => pt_2(text),
        Test::Both(text) => {
            pt_1(text);
            pt_2(text);
        }
    }
}
