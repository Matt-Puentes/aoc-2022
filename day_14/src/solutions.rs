use std::collections::HashSet;
// use std::{thread, time};

// fn print_map(map: &HashSet<(u16, u16)>, sandpos: &(u16, u16)) {
//     let mut dims = (600, 0, 0, 0); // min x, min y, max x, max y
//     for p in map {
//         if p.0 < dims.0 {
//             dims.0 = p.0
//         }
//         if p.0 > dims.2 {
//             dims.2 = p.0
//         }
//         if p.1 > dims.3 {
//             dims.3 = p.1
//         }
//     }
//     // println!("Corners: bottomright: {:?}", dims);
//     for y in dims.1..(dims.3 + 1) {
//         for x in dims.0..(dims.2 + 1) {
//             if (x, y) == *sandpos {
//                 print!("+");
//             } else if map.contains(&(x, y)) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         print!("\n");
//     }
// }

fn read_map(str_input: &str) -> (HashSet<(u16, u16)>, u16) {
    let mut map: HashSet<(_, _)> = HashSet::new();
    let mut height_limit = 0; // if the sand hits this, sim is over
    for l in str_input.lines() {
        let mut prevpoint: Option<(_, _)> = None;
        for p in l.split(" -> ") {
            let (xs, ys) = p.split_once(',').unwrap();
            let x = xs.parse::<u16>().unwrap();
            let y = ys.parse::<u16>().unwrap();

            if y > height_limit {
                height_limit = y
            }

            if let Some((px, py)) = prevpoint.take() {
                let (fromy, toy) = (y.min(py), y.max(py) + 1);
                let (fromx, tox) = (x.min(px), x.max(px) + 1);

                for dy in fromy..toy {
                    map.insert((x, dy));
                }
                for dx in fromx..tox {
                    map.insert((dx, y));
                }
            }
            prevpoint = Some((x, y));
        }
    }
    (map, height_limit)
}

pub fn pt_1(str_input: &str) {
    let (mut map, height_limit) = read_map(str_input);

    let mut c = 0;
    'sandSpawn: loop {
        let mut sand_pos = (500, 0);
        'sandFall: loop {
            let new_pos = (sand_pos.0, sand_pos.1 + 1);
            if !map.contains(&new_pos) {
                sand_pos = new_pos;
            } else if !map.contains(&(new_pos.0 - 1, new_pos.1)) {
                sand_pos = (new_pos.0 - 1, new_pos.1);
            } else if !map.contains(&(new_pos.0 + 1, new_pos.1)) {
                sand_pos = (new_pos.0 + 1, new_pos.1);
            } else {
                // sand has come to rest
                c += 1;
                map.insert(sand_pos);
                break 'sandFall;
            }
            if sand_pos.1 >= height_limit {
                println!("HEIGHT LIMIT REACHED");
                break 'sandSpawn;
            }
        }
    }
    println!("Part 1 result: {}", c)
}

pub fn pt_2(str_input: &str) {
    let (mut map, height_limit) = read_map(str_input);

    let mut c = 0;
    // let mut sand_trail = vec![];
    let start_sand_from = (500, 0);
    'sandSpawn: loop {
        let mut sand_pos = start_sand_from;
        'sandFall: loop {
            let new_pos = (sand_pos.0, sand_pos.1 + 1);
            // if map.contains(&sand_pos) {
            //     if let Some(safe_pos) = sand_trail.pop() {
            //         start_sand_from = safe_pos;
            //         continue 'sandFall;
            //     } else {
            //         println!("Out of safe positions to return to! exiting loop.");
            //         break 'sandSpawn;
            //     }
            // }

            // The sand has hit the floor
            if new_pos.1 == height_limit + 2 {
                c += 1;
                map.insert(sand_pos);
                break 'sandFall;
            }

            // Check sand pos
            if !map.contains(&new_pos) {
                // sand_trail.push(sand_pos);
                sand_pos = new_pos;
            } else if !map.contains(&(new_pos.0 - 1, new_pos.1)) {
                // sand_trail.push(sand_pos);
                sand_pos = (new_pos.0 - 1, new_pos.1);
            } else if !map.contains(&(new_pos.0 + 1, new_pos.1)) {
                // sand_trail.push(sand_pos);
                sand_pos = (new_pos.0 + 1, new_pos.1);
            } else {
                c += 1;
                map.insert(sand_pos);
                if sand_pos == (500, 0) {
                    break 'sandSpawn;
                } else {
                    break 'sandFall;
                }
            }
        }
    }
    println!("Part 2 result: {}", c)
}
