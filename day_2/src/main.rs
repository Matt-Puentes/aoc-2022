mod parse_args;
use parse_args::{parse_args, Test};

pub fn pt_1(str_input: &str) {
    let res: u32 = str_input
        .lines()
        .map(|l| {
            (match l {
                "A Y" | "B Z" | "C X" => 6,
                "A X" | "B Y" | "C Z" => 3,
                "A Z" | "B X" | "C Y" => 0,
                _ => panic!("bad line {:?}", l),
            }) + (l.chars().last().unwrap() as u32 - 87)
        })
        .sum();
    println!("Part 1 result: {:?}", res)
}

pub fn pt_2(str_input: &str) {
    let res: u32 = str_input
        .lines()
        .map(|l| {
            (match l {
                "A Y" | "B X" | "C Z" => 1, //rock
                "A Z" | "B Y" | "C X" => 2, //paper
                "A X" | "B Z" | "C Y" => 3, //scissors
                _ => panic!("bad line!"),
            } + (l.chars().last().unwrap() as u32 - 88) * 3)
        })
        .sum();
    println!("Part 2 result: {:?}", res)
}

pub fn main() {
    println!("Running day 2");
    match parse_args() {
        Test::One(text) => pt_1(text),
        Test::Two(text) => pt_2(text),
        Test::Both(text) => {
            pt_1(text);
            pt_2(text);
        }
    }
}
