use std::collections::HashSet;

fn ord(c: char) -> usize {
    "-abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .find(c)
        .unwrap()
}

pub fn pt_1(str_input: &str) {
    println!(
        "Part 1 result: {:?}",
        str_input
            .lines()
            .map(|s| {
                let mut chars: HashSet<char> = HashSet::new();
                s.chars()
                    .enumerate()
                    .filter_map(|(idx, c)| {
                        if idx < s.len() / 2 {
                            chars.insert(c);
                            None
                        } else if chars.remove(&c) {
                            Some(ord(c))
                        } else {
                            None
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    )
}

pub fn pt_2(str_input: &str) {
    println!(
        "Part 2 result: {:?}",
        str_input
            .lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|chunk| {
                match chunk {
                    [l1, l2, l3] => {
                        let mut chars: HashSet<char> = l1.chars().collect();
                        chars.retain(|c| l2.find(*c).is_some());
                        chars.retain(|c| l3.find(*c).is_some());
                        chars.iter().map(|c| ord(*c)).sum::<usize>()
                    }
                    _ => panic!("Not 3 matches"),
                }
            })
            .sum::<usize>()
    )
}
