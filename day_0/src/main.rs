use std::env;

fn pt_1(str_input: &str) -> usize {
    str_input.len().try_into().unwrap_or(0)
}

fn pt_2(str_input: &str) -> usize {
    str_input.len().try_into().unwrap_or(0)
}

fn main() {
    println!("Running day 0");
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
