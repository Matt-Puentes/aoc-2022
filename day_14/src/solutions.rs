use std::collections::VecDeque;

enum Line {
    Horiz((usize, usize), usize),
    Vert(usize, (usize, usize)),
}

// I guessss I could just make a really big World (1k x 5k or something) and fill the array in only one pass. That would
//  probably also be faster. But I prefer Generic solutions, and it's only 2 iterations to do it this way.
fn build_world(str_input: &str, pt2: bool) -> (Vec<Vec<char>>, (usize, usize)) {
    // y min is always 0
    let mut max_row = 0; // if the sand hits this, sim is over
    let mut min_col = 1000; // So we can shift the coord system over
    let mut max_col = 0; // So we know the width of the world

    // Construct lines while determining bounds of world
    let mut lines: Vec<Line> = vec![];
    for l in str_input.lines() {
        let mut prevpoint: Option<(_, _)> = None;
        for p in l.split(" -> ") {
            let (xs, ys) = p.split_once(',').unwrap();
            let x = xs.parse::<usize>().unwrap();
            let y = ys.parse::<usize>().unwrap();

            if y > max_row {
                max_row = y
            }
            if x < min_col {
                min_col = x
            }
            if x > max_col {
                max_col = x
            }
            if let Some((px, py)) = prevpoint.take() {
                if px == x {
                    lines.push(Line::Vert(x, (y.min(py), y.max(py))));
                } else if py == y {
                    lines.push(Line::Horiz((x.min(px), x.max(px)), y));
                } else {
                    panic!("Diagonal Line ({:?}, {:?}) detected", (x, y), (px, py))
                }
            }
            prevpoint = Some((x, y));
        }
    }

    if pt2 {
        // Give room for the "floor"
        max_row += 2;
        // give a LOT of extra room for sand to pile up
        min_col -= max_row;
        max_col += max_row;
    } else {
        // Give room for sand to fall off
        min_col -= 1;
        max_col += 1;
    }

    // Fill world space with "air" ('.')
    let mut world = vec![vec!['.'; max_col - min_col + 1]; max_row + 1];

    // For each Line, draw walls into world
    for line in lines {
        match line {
            Line::Vert(x, (s, e)) => {
                for y in s..=e {
                    world[y][x - min_col] = '#';
                }
            }
            Line::Horiz((s, e), y) => {
                for x in s..=e {
                    world[y][x - min_col] = '#';
                }
            }
        }
    }

    (world, (max_row, min_col))
}

pub fn pt_1(str_input: &str) {
    let (mut world, (height_limit, min_col)) = build_world(str_input, false);
    let mut c = 0;
    'sandSpawn: loop {
        let mut sand_pos = (0, 500 - min_col);
        'sandFall: loop {
            let new_pos = (sand_pos.0 + 1, sand_pos.1);
            if sand_pos.0 >= height_limit {
                break 'sandSpawn;
            }
            if world[new_pos.0][new_pos.1] == '.' {
                sand_pos = new_pos;
            } else if world[new_pos.0][new_pos.1 - 1] == '.' {
                sand_pos = (new_pos.0, new_pos.1 - 1);
            } else if world[new_pos.0][new_pos.1 + 1] == '.' {
                sand_pos = (new_pos.0, new_pos.1 + 1);
            } else {
                c += 1;
                world[sand_pos.0][sand_pos.1] = 'o';
                break 'sandFall;
            }
        }
    }
    println!("Part 1 result: {}", c)
}

pub fn pt_2(str_input: &str) {
    let (mut world, (height_limit, min_col)) = build_world(str_input, true);
    let mut c = 0;

    // You can just floodfill for pt2, since it'll fill up all possible space
    // Inspired by (Copied from?) https://www.reddit.com/r/adventofcode/comments/zli1rd/comment/j06ea2y/?utm_source=share&utm_medium=web2x&context=3

    let mut pos_queue: VecDeque<(usize, usize)> = Default::default();
    pos_queue.push_back((0, 500 - min_col));
    while let Some(next_pos) = pos_queue.pop_front() {
        if world[next_pos.0][next_pos.1] == '.' {
            c += 1;
            world[next_pos.0][next_pos.1] = 'o';

            let dy = next_pos.0 + 1;

            // If the sand already reached the floor, don't bother checking the points below
            if dy == height_limit {
                continue;
            }

            pos_queue.push_back((dy, next_pos.1));
            pos_queue.push_back((dy, next_pos.1 - 1));
            pos_queue.push_back((dy, next_pos.1 + 1));
        }
    }
    println!("Part 2 result: {}", c)
}
