use std::vec;
mod parse_args;
use parse_args::{parse_args, Test};
#[derive(Debug, PartialEq)]
enum Data {
    Dir(String),
    File(usize),
}

#[derive(Debug)]
struct Node {
    data: Data,
    children: Vec<usize>,
    parent: Option<usize>,
}

pub fn construct_dirs(str_input: &str) -> Vec<usize> {
    // use an "Arena" to store nodes by index. This way they can be mutated while we traverse the tree.
    let mut nodes: Vec<Node> = vec![Node {
        data: Data::Dir("/".to_string()),
        children: Vec::new(),
        parent: None,
    }];
    let mut cursor = 0;

    for cmd in str_input.lines() {
        match cmd.split(" ").collect::<Vec<_>>()[..] {
            ["$", "cd", "/"] => cursor = 0,
            ["$", "cd", ".."] => {
                cursor = nodes[cursor].parent.expect("Node does not have parent");
            }
            ["$", "cd", dir] => {
                cursor = *nodes[cursor]
                    .children
                    .iter()
                    .find(|c| match &nodes[**c].data {
                        Data::Dir(d) => d == dir,
                        _ => false,
                    })
                    .expect(format!("No child {} found for node {:?}", dir, cursor).as_str());
            }
            ["$", "ls"] => {}
            ["dir", name] => {
                let new_idx = nodes.len();
                nodes.push(Node {
                    data: Data::Dir(name.to_string()),
                    children: Vec::new(),
                    parent: Some(cursor),
                });
                nodes[cursor].children.push(new_idx);
            }
            [num, _] => {
                let new_idx = nodes.len();
                nodes.push(Node {
                    data: Data::File(num.parse().unwrap()),
                    children: Vec::new(),
                    parent: Some(cursor),
                });
                nodes[cursor].children.push(new_idx);
            }
            _ => panic!("Bad line {}", cmd),
        }
    }

    // This is an inefficient way of tallying dirs, because we recompute every directory every time.
    fn get_dir_size(nodes: &Vec<Node>, dir: &Node) -> usize {
        dir.children
            .iter()
            .map(|c| match &nodes[*c].data {
                Data::File(s) => *s,
                Data::Dir(_) => get_dir_size(nodes, &nodes[*c]),
            })
            .sum()
    }

    nodes
        .iter()
        .filter_map(|s| match s.data {
            Data::File(_) => None,
            Data::Dir(_) => Some(get_dir_size(&nodes, s)),
        })
        .collect::<Vec<_>>()
}

pub fn pt_1(str_input: &str) {
    let dir_sizes = construct_dirs(str_input);

    println!(
        "Part 1 result: {:?}",
        dir_sizes.iter().filter(|&&s| s < 100000).sum::<usize>()
    );
}

pub fn pt_2(str_input: &str) {
    let dir_sizes = construct_dirs(str_input);
    let needed_free_space = 30000000 - (70000000 - dir_sizes[0]);

    println!(
        "Part 2 result: {:?}",
        dir_sizes.iter().fold(30000000, |a, &e| {
            if e < a && e >= needed_free_space {
                e
            } else {
                a
            }
        })
    );
}

pub fn main() {
    println!("Running day 7");
    match parse_args() {
        Test::One(text) => pt_1(text),
        Test::Two(text) => pt_2(text),
        Test::Both(text) => {
            pt_1(text);
            pt_2(text);
        }
    }
}
