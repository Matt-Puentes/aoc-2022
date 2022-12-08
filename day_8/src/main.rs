mod parse_args;
use parse_args::{parse_args, Test};

pub fn pt_1(str_input: &str) {
    let rows = str_input.lines().count();
    let cols = str_input.lines().next().unwrap().len();

    // Store visibility in matrix
    let mut is_visible = vec![vec![false; cols]; rows];

    // For right/top visibility, we're already iterating over the strings in those directions, so just keep track of the
    //  highest tree so far, and if it's taller than it's visible.

    // For left/down visibility, we need to keep track of what is visible while iterating and tally it at the end.
    //  If we use a stack, we can push trees on when we find them and pop them off if a taller tree comes along
    //  to "block" them.

    // left/right visibility are tracked inside the loop, top/down visibility are tracked outside the loop in vectors
    //  indexed by column index.

    // Tallest tree in each column
    let mut up_highest = vec!['0' as usize - 1; cols];
    // Stacks of tree hight for each column
    let mut down_stacks: Vec<Vec<(usize, usize)>> = vec![Vec::new(); cols];

    for (ri, l) in str_input.lines().enumerate() {
        // Keeping track of the highest so far this row
        let mut highest = '0' as usize - 1;
        // Keeping track of the tree size order from the left
        let mut talltreestack: Vec<(usize, usize)> = Vec::new(); //index, height

        for (ci, c) in l.chars().enumerate() {
            // println!("char: {}, as usize: {} ", c, c as usize);
            // Check right -> left visibility
            if c as usize > highest {
                highest = c as usize;
                is_visible[ri][ci] = true;
            }
            // Check up -> down visibility
            if c as usize > up_highest[ci] {
                up_highest[ci] = c as usize;
                is_visible[ri][ci] = true;
            }
            // Check left -> right visibility
            loop {
                // check the tree stack
                match talltreestack.pop() {
                    Some(t) => {
                        if (c as usize) < t.1 {
                            talltreestack.push(t);
                            talltreestack.push((ci, c as usize));
                            break;
                        }
                    }
                    None => {
                        talltreestack.push((ci, c as usize));
                        break;
                    }
                }
            }
            loop {
                // check the tree stack
                match down_stacks[ci].pop() {
                    Some(t) => {
                        if (c as usize) < t.1 {
                            down_stacks[ci].push(t);
                            down_stacks[ci].push((ri, c as usize));
                            break;
                        }
                    }
                    None => {
                        down_stacks[ci].push((ri, c as usize));
                        break;
                    }
                }
            }
        }
        for (ti, _) in talltreestack {
            is_visible[ri][ti] = true;
        }
    }

    for (ci, col) in down_stacks.iter().enumerate() {
        for (ri, _) in col {
            is_visible[*ri][ci] = true;
        }
    }

    println!(
        "Part 1 result: {}",
        is_visible
            .iter()
            .map(|r| r.iter().filter(|s| **s).count())
            .sum::<usize>()
    )
}

pub fn pt_2(str_input: &str) {
    // Build full input into 2d vector, so we can index easily later
    let matrix = str_input
        .lines()
        .map(|r| r.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut highest_score = 0;
    // Do the walk for each character
    for (ri, l) in str_input.lines().enumerate() {
        for (ci, c) in l.chars().enumerate() {
            let mut score = 1;

            // The four directions- up, left, down, right.
            let dirs: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
            for (rd, cd) in dirs {
                let mut count = 0;
                // Step in that direction...
                for i in 1.. {
                    // Check that the index is valid...
                    let r_i = ri as i32 + (rd * i);
                    let c_i = ci as i32 + (cd * i);
                    if r_i >= 0
                        && r_i < matrix.len() as i32
                        && c_i >= 0
                        && c_i < matrix[r_i as usize].len() as i32
                    {
                        // Check trees
                        if matrix[r_i as usize][c_i as usize] < c {
                            // Tree passed over
                            count += 1;
                        } else {
                            // Tree hit
                            count += 1;
                            score *= count;
                            break;
                        }
                    } else {
                        // Index invalid, edge hit
                        score *= count;
                        break;
                    }
                }
            }
            if score > highest_score {
                highest_score = score;
            }
        }
    }
    println!("Part 2 result: {}", highest_score)
}

pub fn main() {
    println!("Running day 8");
    match parse_args() {
        Test::One(text) => pt_1(text),
        Test::Two(text) => pt_2(text),
        Test::Both(text) => {
            pt_1(text);
            pt_2(text);
        }
    }
}
