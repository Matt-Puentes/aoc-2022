mod parse_args;
use core::panic;
use std::vec;

use parse_args::{parse_args, Test};

fn build_stacks(boxes: &str) -> Vec<Vec<char>> {
    let mut boxes = boxes.lines().rev();

    // build vector of stacks
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..((boxes.next().unwrap().len() / 4) + 1) {
        stacks.push(Vec::new());
    }

    // Populate stacks
    for l in boxes {
        let mut chars = l.chars();
        for i in 0..stacks.len() {
            match (chars.next(), chars.next(), chars.next()) {
                (_, Some(' '), _) => (),
                (_, Some(c), _) => stacks.get_mut(i).unwrap().push(c),
                _ => panic!("No char found"),
            }
            // throw away space between columns
            chars.next();
        }
    }
    stacks
}

fn pt_1(str_input: &str) {
    let (boxes, instructions) = str_input.split_once("\n\n").unwrap();
    let mut stacks = build_stacks(boxes);
    for instruction in instructions.lines() {
        let mut chars = instruction.split(' ');
        let move_amt = chars.nth(1).unwrap().parse::<usize>().unwrap();
        let from_stack = chars.nth(1).unwrap().parse::<usize>().unwrap() as usize;
        let to_stack = chars.nth(1).unwrap().parse::<usize>().unwrap() as usize;
        for _ in 0..move_amt {
            let c = stacks
                .get_mut(from_stack - 1)
                .unwrap()
                .pop()
                .expect("Stack is empty");
            stacks.get_mut(to_stack - 1).unwrap().push(c)
        }
    }
    println!(
        "Part 1 result: {}",
        stacks
            .iter_mut()
            .map(|s| s.last().unwrap())
            .collect::<String>()
    )
}

fn pt_2(str_input: &str) {
    let (boxes, instructions) = str_input.split_once("\n\n").unwrap();
    let mut stacks = build_stacks(boxes);
    for instruction in instructions.lines() {
        let mut chars = instruction.split(' ');
        let move_amt = chars.nth(1).unwrap().parse::<usize>().unwrap();
        let from_stack = chars.nth(1).unwrap().parse::<usize>().unwrap() as usize;
        let to_stack = chars.nth(1).unwrap().parse::<usize>().unwrap() as usize;
        let from_vec = stacks.get_mut(from_stack - 1).unwrap();
        let t = from_vec
            .drain(from_vec.len() - (move_amt)..)
            .collect::<Vec<_>>();
        stacks.get_mut(to_stack - 1).unwrap().extend(t);
    }
    println!(
        "Part 2 result: {}",
        stacks
            .iter_mut()
            .map(|s| s.last().unwrap())
            .collect::<String>()
    )
}

pub fn main() {
    println!("Running day 5");
    match parse_args() {
        Test::One(text) => pt_1(text),
        Test::Two(text) => pt_2(text),
        Test::Both(text) => {
            pt_1(text);
            pt_2(text);
        }
    }
}
