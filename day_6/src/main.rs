mod parse_args;
use std::collections::HashSet;
use std::time::Instant;

use parse_args::{parse_args, Test};

fn solution(str_input: &str, win_size: usize) -> usize {
    for (idx, chars) in str_input.as_bytes().windows(win_size).enumerate() {
        if !chars
            .iter()
            .enumerate()
            .any(|(char_idx, c)| chars[char_idx + 1..].iter().any(|c2| c == c2))
        {
            return idx + win_size;
        }
    }
    0
}

fn pt_1(str_input: &str) {
    // Origionally i did this solution with manual checking for duplicates c[0] != c[1] && c[0] != c[2] ... etc
    println!("Part 1 result: {}", solution(str_input, 4));
}

fn pt_2(str_input: &str) {
    println!("Part 2 result: {}", solution(str_input, 14))
}

pub fn main() {
    println!("Running day 6");
    match parse_args() {
        Test::One(text) => pt_1(text),
        Test::Two(text) => pt_2(text),
        Test::Both(text) => {
            pt_1(text);
            pt_2(text);
        }
    }
}

// I was resistant to use hsahmaps, but a friend told set creation is only n log n. But when i ran tests,
// there was crazy overhead for the hashmap (or something), so the og solution is a lot faster.
fn _bad_solution(str_input: &str, win_size: usize) -> usize {
    for (idx, chars) in str_input.as_bytes().windows(win_size).enumerate() {
        let mut set: HashSet<u8> = HashSet::new();
        if !chars.iter().any(|c| !set.insert(*c)) {
            return idx + win_size;
        }
    }
    0
}

fn _run_benchmark(str_input: &str) {
    let mut results: Vec<(u128, u128)> = vec![];
    for win_size in 1..20 {
        let now = Instant::now();
        for _ in 0..1000 {
            solution(str_input, win_size);
        }

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed.as_nanos());

        println!("== win_size {} ==", win_size);
        // Code block to measure.
        let now1 = Instant::now();
        for _ in 0..1000 {
            _bad_solution(str_input, win_size);
        }
        let elapsed1 = now1.elapsed();
        println!("hashmap Elapsed: {:.2?}", elapsed1.as_nanos());
        results.push((elapsed.as_nanos(), elapsed1.as_nanos()))
    }
    println!("results: {:?}", results);
}
