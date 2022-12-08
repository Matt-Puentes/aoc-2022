use std::vec;

#[derive(Debug)]
struct Node {
    name: String,
    file_size: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}

pub fn construct_dirs(str_input: &str) -> Vec<usize> {
    // use an "Arena" to store nodes by index. This way they can be mutated while we traverse the tree.
    let mut nodes: Vec<Node> = vec![Node {
        name: "/".to_string(),
        file_size: 0,
        children: vec![],
        parent: None,
    }];
    let mut cursor = 0;

    for cmd in str_input.lines() {
        match cmd.split(" ").collect::<Vec<_>>()[..] {
            ["$", "ls"] => { /* lmao */ }
            ["$", "cd", "/"] => cursor = 0,
            ["$", "cd", ".."] => cursor = nodes[cursor].parent.expect("Node does not have parent"),
            ["$", "cd", dir] => {
                cursor = *nodes[cursor]
                    .children
                    .iter()
                    .find(|c| nodes[**c].name == dir)
                    .expect(format!("No child {} found for node {:?}", dir, cursor).as_str())
            }
            ["dir", name] => {
                let new_idx = nodes.len();
                nodes.push(Node {
                    name: name.to_string(),
                    children: vec![],
                    file_size: 0,
                    parent: Some(cursor),
                });
                nodes[cursor].children.push(new_idx);
            }
            [num, _] => nodes[cursor].file_size += num.parse::<usize>().unwrap(),
            _ => panic!("Bad line {}", cmd),
        }
    }

    // This is an inefficient way of tallying dirs, because we recompute every directory every time.
    fn get_dir_size(nodes: &Vec<Node>, dir: &Node) -> usize {
        dir.file_size
            + dir
                .children
                .iter()
                .map(|s| get_dir_size(nodes, &nodes[*s]))
                .sum::<usize>()
    }

    nodes
        .iter()
        .map(|s| get_dir_size(&nodes, s))
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
