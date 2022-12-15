const _EXAMPLE_Y: i32 = 10;
const _EXAMPLE_MAX: i32 = 20;
const _INPUT_Y: i32 = 2_000_000;
const _INPUT_MAX: i32 = 4_000_000;

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

    let row = _EXAMPLE_Y;
    // let row = _INPUT_Y;
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
    let mut answer: Option<i64> = None;

    // Never got a good solution. Revisit when you have time and look at existing solutions

    let range = 0.._INPUT_MAX;
    // let range = 0.._EXAMPLE_MAX;
    'outer: for row in range {
        let mut coverages: Vec<(i32, i32)> = vec![];
        for Sensor { pos, radius } in &sensors {
            // the amount of columns a sensor covers at that radius
            let cov_start = pos.0 - (radius - (pos.1 - row).abs());
            let cov_end = pos.0 + (radius - (pos.1 - row).abs()) + 1;
            if cov_start < cov_end {
                coverages.push((cov_start, cov_end));
            }
        }

        // merge overlapping coverages
        coverages.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        let mut merged_coverages: Vec<(i32, i32)> = vec![coverages[0].clone()];
        for i in 1..coverages.len() {
            let cov = coverages[i].clone();
            let idx = merged_coverages.len() - 1;
            if cov.0 >= merged_coverages[idx].0 && cov.0 <= merged_coverages[idx].1 {
                // println!(
                //     "coverage {:?} eats coverage {:?}",
                //     merged_coverages[idx], cov
                // );
                merged_coverages[idx].1 = cov.1.max(merged_coverages[idx].1);
            } else {
                // println!("cov gets pushed onto stack");
                merged_coverages.push(cov);
            }
        }
        if merged_coverages.len() == 2 {
            println!("Found it: {}, {}", merged_coverages[0].1, row);
            answer = Some((merged_coverages[0].1 as i64 * 4000000 as i64) + row as i64);
            break 'outer;
        }

        // println!("{}: {:?}", row, merged_coverages);
    }

    println!("Part 2 result: {:?}", answer.expect("no answer for pt 2"))
}
