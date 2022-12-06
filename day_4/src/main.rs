mod parse_args;
use parse_args::{parse_args, Test};

pub fn pt_1(str_input: &str) {
    println!(
        "Part 1 result: {:?}",
        str_input
            .lines()
            .map(|s| {
                let (s1, s2) = s.split_once(',').unwrap();
                let (l1, l2) = s1.split_once('-').unwrap();
                let (r1, r2) = s2.split_once('-').unwrap();
                (
                    (l1.parse::<u32>().unwrap(), l2.parse::<u32>().unwrap()),
                    (r1.parse::<u32>().unwrap(), r2.parse::<u32>().unwrap()),
                )
            })
            .filter(|((l1, l2), (r1, r2))| (r1 <= l1 && r2 >= l2 || l1 <= r1 && l2 >= r2))
            .collect::<Vec<_>>()
            .len()
    )
}

pub fn pt_2(str_input: &str) {
    println!(
        "Part 1 result: {:?}",
        str_input
            .lines()
            .map(|s| {
                let (s1, s2) = s.split_once(',').unwrap();
                let (l1, l2) = s1.split_once('-').unwrap();
                let (r1, r2) = s2.split_once('-').unwrap();
                (
                    (l1.parse::<u32>().unwrap(), l2.parse::<u32>().unwrap()),
                    (r1.parse::<u32>().unwrap(), r2.parse::<u32>().unwrap()),
                )
            })
            .filter(|((l1, l2), (r1, r2))| (l2 >= r1 && l2 <= r2
                || l1 >= r1 && l1 <= r2
                || r2 >= l1 && r2 <= l2
                || r1 >= l1 && r1 <= l2))
            .collect::<Vec<_>>()
            .len()
    )
}

pub fn main() {
    println!("Running day 4");
    match parse_args() {
        Test::One(text) => pt_1(text),
        Test::Two(text) => pt_2(text),
        Test::Both(text) => {
            pt_1(text);
            pt_2(text);
        }
    }
}
