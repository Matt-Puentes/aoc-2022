use std::vec;

pub fn pt_1(str_input: &str) {
    let mut cycle: i32 = 1;
    let mut register: i32 = 1;

    let mut signal_strenghts = vec![];
    for l in str_input.lines() {
        if l == "noop" {
            if (cycle - 20) % 40 == 0 {
                signal_strenghts.push(register * cycle as i32)
            }
            cycle += 1;
        } else {
            match l.split_once(' ') {
                Some(("addx", num)) => {
                    for i in 0..2 {
                        if ((cycle + i) - 20) % 40 == 0 {
                            signal_strenghts.push(register * (cycle + i) as i32)
                        }
                    }
                    register += num.parse::<i32>().unwrap();
                    cycle += 2;
                }
                _ => panic!("Bad line!"),
            }
        }
    }
    println!("Part 1 result: {}", signal_strenghts.iter().sum::<i32>())
}

pub fn pt_2(str_input: &str) {
    let mut cycle: i32 = 0;
    let mut register: i32 = 1;

    let mut screen: [[bool; 40]; 6] = [[false; 40]; 6];
    for l in str_input.lines() {
        let mut row = ((cycle) / 40) as usize;
        let mut col = ((cycle) % 40) as usize;
        let mut val = (register - ((cycle) % 40)).abs() <= 1;
        if l == "noop" {
            screen[row][col] = val;
            cycle += 1;
        } else {
            match l.split_once(' ') {
                Some(("addx", num)) => {
                    for i in 0..2 {
                        row = ((cycle + i) / 40) as usize;
                        col = ((cycle + i) % 40) as usize;
                        val = (register - ((cycle + i) % 40)).abs() <= 1;
                        screen[row][col] = val;
                    }
                    register += num.parse::<i32>().unwrap();
                    cycle += 2;
                }
                _ => panic!("Bad line!"),
            }
        }
    }
    println!(
        "Part 2 result:\n{}",
        screen
            .iter()
            .map(|s| s
                .iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    )
}
