mod parse_args;
use parse_args::{parse_args, Test};

fn pt_1(str_input: &str) -> usize {
    str_input
        .split("\n\n") // Split into blocks
        .map(|s| {
            s.lines()
                .map(|s| s.trim().parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .expect("List is empty")
}

fn pt_2(str_input: &str) -> usize {
    let mut vec: Vec<usize> = str_input
        .split("\n\n") // Split into blocks
        .map(|s| {
            s.lines()
                .map(|s| s.trim().parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect();
    vec.sort_unstable();
    vec.reverse();
    vec[0] + vec[1] + vec[2]
}

fn main() {
    println!("Running day 0");
    match parse_args() {
        Test::One(text) => println!("Part 1 result: {}", pt_2(text)),
        Test::Two(text) => println!("Part 2 result: {}", pt_2(text)),
        Test::Both(text) => println!(
            "Part 1 result: {}, Part 2 result: {}",
            pt_1(text),
            pt_2(text)
        ),
    }
}
