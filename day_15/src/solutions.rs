const EXAMPLE_Y: i32 = 10;
const EXAMPLE_MAX: i32 = 20;
// const INPUT_Y: i32 = 2000000;
const INPUT_MAX: i32 = 4_000_000;

#[derive(Debug)]
struct Sensor {
    pos: (i32, i32),
    radius: i32,
}

fn dist(pos1: (i32, i32), pos2: (i32, i32)) -> i32 {
    (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs()
}

fn build_sensors(str_input: &str) -> Vec<Sensor> {
    let mut sensors: Vec<Sensor> = vec![];
    // let mut bounds: ((i32, i32), (i32, i32)) = ((i32::MAX, 0), (i32::MAX, 0));
    for l in str_input.lines() {
        let (sensordef, beacondef) = l.split_once(": ").unwrap();
        let (sensorxdef, sensorydef) = sensordef.split_once(", ").unwrap();
        let (beaconxdef, beaconydef) = beacondef.split_once(", ").unwrap();
        let sensor_pos: (i32, i32) = (
            sensorxdef[12..].parse().unwrap(),
            sensorydef[2..].parse().unwrap(),
        );
        let beacon_pos: (i32, i32) = (
            beaconxdef[23..].parse().unwrap(),
            beaconydef[2..].parse().unwrap(),
        );
        let radius = dist(sensor_pos, beacon_pos);
        sensors.push(Sensor {
            pos: sensor_pos,
            radius: radius,
        });
    }
    sensors
}

pub fn pt_1(str_input: &str) {
    let sensors = build_sensors(str_input);

    let row = EXAMPLE_Y;
    // let row = INPUT_Y;
    let mut coverages: Vec<(i32, i32)> = vec![];
    for Sensor { pos, radius } in sensors {
        // the amount of columns a sensor covers at that radius
        let cov_start = pos.0 - (radius - (pos.1 - row).abs());
        let cov_end = pos.0 + (radius - (pos.1 - row).abs());
        if cov_start < cov_end {
            coverages.push((cov_start, cov_end));
        }
    }

    // merge overlapping coverages
    coverages.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let mut merged_coverages: Vec<(i32, i32)> = vec![coverages[0].clone()];
    for i in 1..coverages.len() {
        let idx = merged_coverages.len() - 1;
        if coverages[i].0 >= merged_coverages[idx].0 && coverages[i].0 <= merged_coverages[idx].1 {
            merged_coverages[idx].1 = coverages[i].1.max(merged_coverages[idx].1);
        }
    }

    println!(
        "Part 1 result: {}",
        merged_coverages.iter().map(|c| c.1 - c.0).sum::<i32>()
    )
}

pub fn pt_2(str_input: &str) {
    let sensors = build_sensors(str_input);
    let mut answer: Option<(i32, i32)> = None;

    // Never got a good solution. Revisit when you have time and look at existing solutions

    // let max = EXAMPLE_MAX;
    // let max = INPUT_MAX;
    // 'outer: for Sensor { pos, radius } in &sensors {
    //     // get positions outside edge of sensor
    //     for y in 0..=(radius * 2) {
    //         let row = (pos.1 - radius) + y;
    //         let x1 = pos.0 - (radius - (pos.1 - row).abs()) - 1;
    //         let x2 = pos.0 + (radius - (pos.1 - row).abs()) + 1;
    //         if y > 0 && y < max && x1 > 0 && x1 < max {
    //             if sensors
    //                 .iter()
    //                 .map(|c| dist((x1, y), c.pos) > c.radius)
    //                 .all(|b| b)
    //             {
    //                 println!("1 FOUND IT {:?}", (x1, y));
    //                 answer = Some((x1, y));
    //                 break 'outer;
    //             }
    //         }
    //         if y > 0 && y < max && x2 > 0 && x2 < max {
    //             if sensors
    //                 .iter()
    //                 .map(|c| dist((x2, y), c.pos) > c.radius)
    //                 .all(|b| b)
    //             {
    //                 println!("2 FOUND IT {:?}", (x2, y));
    //                 answer = Some((x2, y));
    //                 break 'outer;
    //             }
    //         }
    //     }
    //     // far edges
    //     let p1 = (pos.0, pos.1 - radius - 1);
    //     let p2 = (pos.0, pos.1 + radius + 1);
    //     if p1.0 > 0 && p1.0 < max && p1.1 > 0 && p1.1 < max {
    //         if sensors
    //             .iter()
    //             .map(|c| dist((p1.0, p1.1), c.pos) > c.radius)
    //             .all(|b| b)
    //         {
    //             println!("1 FOUND IT {:?}", (p1.0, p1.1));
    //             answer = Some((p1.0, p1.1));
    //             break 'outer;
    //         }
    //     }
    //     if p2.1 > 0 && p2.1 < max && p2.0 > 0 && p2.0 < max {
    //         if sensors
    //             .iter()
    //             .map(|c| dist((p2.0, p2.1), c.pos) > c.radius)
    //             .all(|b| b)
    //         {
    //             println!("2 FOUND IT {:?}", (p2.0, p2.1));
    //             answer = Some((p2.0, p2.1));
    //             break 'outer;
    //         }
    //     }
    // }
    println!("Part 2 result: {:?}", answer.expect("no answer for pt 2"))
}
