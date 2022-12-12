use std::collections::VecDeque;

#[derive(Debug)]
enum Op {
    MultSame,
    AddSame,
    MultNum(usize),
    AddNum(usize),
}
#[derive(Debug)]
struct Monkey {
    pub items: VecDeque<usize>,
    pub operation: Op,
    pub test: usize,
    pub true_send: usize,
    pub false_send: usize,
}

fn make_monkeys(str_input: &str) -> Vec<Monkey> {
    let lines = str_input.lines().collect::<Vec<_>>();
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkeydef in lines.chunks(7) {
        // id starts at 0 and goes up so we don't need to read it
        monkeys.push(Monkey {
            items: monkeydef[1][18..]
                .split(", ")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<VecDeque<_>>(),
            operation: match monkeydef[2][19..].split(' ').collect::<Vec<_>>()[..] {
                ["old", "+", "old"] => Op::AddSame,
                ["old", "*", "old"] => Op::MultSame,
                ["old", "+", n] | [n, "+", "old"] => Op::AddNum(n.parse::<usize>().unwrap()),
                ["old", "*", n] | [n, "*", "old"] => Op::MultNum(n.parse::<usize>().unwrap()),
                _ => panic!("bad operator"),
            },
            test: monkeydef[3][21..].parse::<usize>().unwrap(),
            true_send: monkeydef[4][29..].parse::<usize>().unwrap(),
            false_send: monkeydef[5][30..].parse::<usize>().unwrap(),
        });
    }
    monkeys
}

pub fn pt_1(str_input: &str) {
    let mut monkeys = make_monkeys(str_input);
    let mut monkey_inspect_count = vec![0; monkeys.len()];

    for _ in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            while let Some(worry) = &monkeys[monkey_idx].items.pop_front() {
                monkey_inspect_count[monkey_idx] += 1;
                let new_worry = match &monkeys[monkey_idx].operation {
                    Op::MultSame => worry * worry,
                    Op::AddSame => worry + worry,
                    Op::MultNum(n) => worry * n,
                    Op::AddNum(n) => worry + n,
                } / 3;

                let idx = if new_worry % &monkeys[monkey_idx].test == 0 {
                    monkeys[monkey_idx].true_send
                } else {
                    monkeys[monkey_idx].false_send
                };
                monkeys[idx].items.push_back(new_worry);
            }
        }
    }
    monkey_inspect_count.sort_unstable_by(|a, b| b.cmp(a));
    println!(
        "Part 1 result: {:?}",
        monkey_inspect_count[0] * monkey_inspect_count[1]
    );
}

pub fn pt_2(str_input: &str) {
    let mut monkeys = make_monkeys(str_input);

    let supermodulo = monkeys.iter().map(|m| m.test).product::<usize>();

    let mut monkey_inspect_count = vec![0; monkeys.len()];

    for _ in 0..10000 {
        for monkey_idx in 0..monkeys.len() {
            while let Some(worry) = monkeys[monkey_idx].items.pop_front() {
                monkey_inspect_count[monkey_idx] += 1;

                let new_worry = match monkeys[monkey_idx].operation {
                    Op::MultSame => worry * worry,
                    Op::AddSame => worry + worry,
                    Op::MultNum(n) => worry * n,
                    Op::AddNum(n) => worry + n,
                } % supermodulo;

                let idx = if new_worry % &monkeys[monkey_idx].test == 0 {
                    monkeys[monkey_idx].true_send
                } else {
                    monkeys[monkey_idx].false_send
                };
                monkeys[idx].items.push_back(new_worry);
            }
        }
    }
    monkey_inspect_count.sort_unstable_by(|a, b| b.cmp(a));
    println!(
        "Part 2 result: {:?}",
        monkey_inspect_count[0] as u128 * monkey_inspect_count[1] as u128
    );
}
