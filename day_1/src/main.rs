use std::env;

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
    println!("Running day 1");
    parse_args();
}

fn parse_args() {
    let args: Vec<String> = env::args().collect();
    let text = if args.len() > 2 && args[2].eq("example") {
        include_str!("./example.txt")
    } else {
        include_str!("./input.txt")
    };

    if args.len() < 2 {
        println!("Part 1 result: {}", pt_1(text));
        println!("Part 2 result: {}", pt_2(text));
    } else {
        match &args[1].parse() {
            Ok(1) => println!("Part 1 result: {}", pt_1(text)),
            Ok(2) => println!("Part 2 result: {}", pt_2(text)),
            Ok(0) => {
                println!("Part 1 result: {}", pt_1(text));
                println!("Part 2 result: {}", pt_2(text));
            }
            _ => panic!("invalid test number"),
        }
    }
}
