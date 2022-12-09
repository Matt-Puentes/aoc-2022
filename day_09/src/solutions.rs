use std::collections::HashSet;

pub fn pt_1(str_input: &str) {
    let mut headpos: [i32; 2] = [0, 0];
    let mut tailpos: [i32; 2] = [0, 0];

    let mut tailpos_tracker: HashSet<[i32; 2]> = HashSet::new();
    tailpos_tracker.insert([0, 0]);
    for l in str_input.lines() {
        if let Some((d, n)) = l.split_once(' ') {
            // println!("{}", l);
            let dir = match d {
                "R" => [1, 0],
                "U" => [0, 1],
                "L" => [-1, 0],
                "D" => [0, -1],
                _ => panic!("Invalid line {}", l),
            };
            for _ in 0..n.parse::<usize>().unwrap() {
                headpos[0] += dir[0];
                headpos[1] += dir[1];

                let hd = headpos[0] - tailpos[0];
                let vd = headpos[1] - tailpos[1];
                if hd.abs() > 1 || vd.abs() > 1 && hd.abs() > 0 {
                    if hd.is_positive() {
                        tailpos[0] += 1
                    } else {
                        tailpos[0] -= 1
                    }
                }
                if vd.abs() > 1 || hd.abs() > 1 && vd.abs() > 0 {
                    if vd.is_positive() {
                        tailpos[1] += 1
                    } else {
                        tailpos[1] -= 1
                    }
                }
                tailpos_tracker.insert(tailpos);
            }
        }
    }

    println!("Part 1 result: {}", tailpos_tracker.len())
}

pub fn pt_2(str_input: &str) {
    let mut rope: [[i32; 2]; 10] = [[0, 0]; 10];
    let mut tailpos_tracker: HashSet<[i32; 2]> = HashSet::new();
    tailpos_tracker.insert([0, 0]);

    for l in str_input.lines() {
        if let Some((d, n)) = l.split_once(' ') {
            let dir = match d {
                "R" => [1, 0],
                "U" => [0, 1],
                "L" => [-1, 0],
                "D" => [0, -1],
                _ => panic!("Invalid line {}", l),
            };
            for _ in 0..n.parse::<usize>().unwrap() {
                rope[0][0] += dir[0];
                rope[0][1] += dir[1];

                for i in 1..rope.len() {
                    let hd = rope[i - 1][0] - rope[i][0];
                    let vd = rope[i - 1][1] - rope[i][1];
                    if hd.abs() > 1 || vd.abs() > 1 && hd.abs() > 0 {
                        if hd.is_positive() {
                            rope[i][0] += 1
                        } else {
                            rope[i][0] -= 1
                        }
                    }
                    if vd.abs() > 1 || hd.abs() > 1 && vd.abs() > 0 {
                        if vd.is_positive() {
                            rope[i][1] += 1
                        } else {
                            rope[i][1] -= 1
                        }
                    }
                }
                tailpos_tracker.insert(rope[9]);
            }
        }
    }

    println!("Part 2 result: {}", tailpos_tracker.len())
}
