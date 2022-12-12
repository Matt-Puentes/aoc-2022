use pathfinding::prelude::astar;

pub fn pt_1(str_input: &str) {
    let mut heightmap: Vec<Vec<i8>> = vec![];
    let mut startpos = (0, 0);
    let mut endpos = (0, 0);
    let mut rows = 0;
    let mut cols = 0;

    for (row, l) in str_input.lines().enumerate() {
        heightmap.push(Vec::new());
        for (col, c) in l.chars().enumerate() {
            match c {
                'S' => {
                    startpos = (row as i32, col as i32);
                    heightmap[row].push('a' as i8)
                }
                'E' => {
                    endpos = (row as i32, col as i32);
                    heightmap[row].push('z' as i8)
                }
                c => heightmap[row].push(c as i8),
            }
            cols = (col + 1) as i32;
        }
        rows = (row + 1) as i32;
    }

    println!("{:?}", startpos);

    println!(
        "Part 1 result: {:?}",
        astar(
            &startpos,
            |&pos| {
                vec![(1, 0), (0, 1), (-1, 0), (0, -1)]
                    .iter()
                    .filter_map(|d| {
                        let newpos = (pos.0 + d.0, pos.1 + d.1);
                        if (newpos.0 < rows)
                            && (newpos.0 >= 0)
                            && (newpos.1 < cols)
                            && (newpos.1 >= 0)
                            && heightmap[newpos.0 as usize][newpos.1 as usize]
                                - heightmap[pos.0 as usize][pos.1 as usize]
                                <= 1
                        {
                            Some((newpos, 1))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            },
            |&_p| 1,
            |&p| p == endpos,
        )
        .expect("No path found")
        .1
    );
}

pub fn pt_2(str_input: &str) {
    let mut heightmap: Vec<Vec<i8>> = vec![];
    let mut start_options: Vec<(i32, i32)> = vec![];
    let mut endpos: (i32, i32) = (0, 0);
    let mut rows = 0;
    let mut cols = 0;

    for (row, l) in str_input.lines().enumerate() {
        heightmap.push(Vec::new());
        for (col, c) in l.chars().enumerate() {
            match c {
                'S' | 'a' => {
                    start_options.push((row as i32, col as i32));
                    heightmap[row].push('a' as i8)
                }
                'E' => {
                    endpos = (row as i32, col as i32);
                    heightmap[row].push('z' as i8)
                }
                c => heightmap[row].push(c as i8),
            }
            cols = (col + 1) as i32;
        }
        rows = (row + 1) as i32;
    }

    println!(
        "Part 2 result: {:?}",
        astar(
            &endpos,
            |&pos| {
                vec![(1, 0), (0, 1), (-1, 0), (0, -1)]
                    .iter()
                    .filter_map(|d| {
                        let newpos = (pos.0 + d.0, pos.1 + d.1);
                        if (newpos.0 < rows)
                            && (newpos.0 >= 0)
                            && (newpos.1 < cols)
                            && (newpos.1 >= 0)
                        {
                            if heightmap[pos.0 as usize][pos.1 as usize]
                                - heightmap[newpos.0 as usize][newpos.1 as usize]
                                <= 1
                            {
                                Some((newpos, 1))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            },
            |&_p| 1,
            |&p| start_options.contains(&p),
        )
        .expect("No path found")
        .1
    )
}
