use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Array(Vec<Value>),
    Num(usize),
}

// I also did this with serde_json, which was ~3ms faster faster, but I wanted to see if I could deal with the list
//  stacking logic in rust.
fn parse_packet(packet_def: &str) -> Value {
    let mut buff: Vec<char> = vec![];
    let mut lists: Vec<Value> = vec![];
    for c in packet_def.chars() {
        match c {
            '[' => lists.push(Value::Array(vec![])),
            ']' => {
                if let Some(Value::Array(mut list)) = lists.pop() {
                    if !buff.is_empty() {
                        let num = Value::Num(
                            (&buff)
                                .into_iter()
                                .collect::<String>()
                                .parse::<usize>()
                                .unwrap(),
                        );
                        buff.clear();
                        list.push(num);
                    }
                    if let Some(Value::Array(mut plist)) = lists.pop() {
                        plist.push(Value::Array(list));
                        lists.push(Value::Array(plist));
                    } else {
                        return Value::Array(list);
                    }
                } else {
                    panic!("No list to end")
                }
            }
            ',' => {
                if !buff.is_empty() {
                    let num = Value::Num(
                        (&buff)
                            .into_iter()
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap(),
                    );
                    buff.clear();
                    if let Some(Value::Array(mut list)) = lists.pop() {
                        list.push(num);
                        lists.push(Value::Array(list));
                    }
                }
            }
            c => buff.push(c),
        }
    }
    panic!("finished function without returning")
}

fn packet_cmp(packet1: &Value, packet2: &Value) -> Ordering {
    match (packet1, packet2) {
        // Arrays compare each of their elements, one-by-one, until one list depletes first.
        (Value::Array(a1), Value::Array(a2)) => {
            let mut a1_iter = a1.iter();
            let mut a2_iter = a2.iter();
            loop {
                match (a1_iter.next(), a2_iter.next()) {
                    (None, None) => break Ordering::Equal,
                    (None, Some(_)) => break Ordering::Less,
                    (Some(_), None) => break Ordering::Greater,
                    (Some(v1), Some(v2)) => match packet_cmp(v1, v2) {
                        Ordering::Equal => continue,
                        // If an element within the list decides the ordering, break
                        r => break r,
                    },
                }
            }
        }
        // I wish I could do this without cloning, but im not sure how I could. cloning an enum(u64) can't be that slow
        //  though, right?
        (Value::Num(_), Value::Array(_)) => {
            packet_cmp(&Value::Array(vec![packet1.clone()]), packet2)
        }
        (Value::Array(_), Value::Num(_)) => {
            packet_cmp(packet1, &Value::Array(vec![packet2.clone()]))
        }
        (Value::Num(n1), Value::Num(n2)) => n1.cmp(&n2),
    }
}

pub fn pt_1(str_input: &str) {
    let mut counter = 0;
    let mut lines = str_input.lines();
    let mut group = 0;
    loop {
        group += 1;

        // Grab lines in groups of 3, the third we'll use to test for the EOF
        let packetdef1 = lines.next().unwrap();
        let packetdef2 = lines.next().unwrap();
        let blank = lines.next();

        // Parse packets
        let packet1 = parse_packet(packetdef1);
        let packet2 = parse_packet(packetdef2);

        // if packet1 < packet 2, the ordering is correct, add the index + 1 to the counter.
        match packet_cmp(&packet1, &packet2) {
            Ordering::Less => counter += group,
            Ordering::Greater => (),
            Ordering::Equal => panic!("No decision made"),
        }

        // If it's the end of the file, we're done
        if let None = blank {
            break;
        }
    }

    println!("Part 1 result: {}", counter)
}

pub fn pt_2(str_input: &str) {
    // pt2-specific marker values
    let marker1 = parse_packet("[[6]]");
    let marker2 = parse_packet("[[2]]");

    // Build vec of packets
    let mut packets: Vec<Value> = vec![marker1.clone(), marker2.clone()];
    for packetdef in str_input.lines() {
        if !packetdef.is_empty() {
            packets.push(parse_packet(packetdef));
        }
    }

    // Sort vector using packet comparison method
    packets.sort_unstable_by(|a, b| packet_cmp(a, b));

    // calculate index(marker1) * index(marker2)
    let mut c = 1;
    for (i, p) in packets.iter().enumerate() {
        if *p == marker1 || *p == marker2 {
            c *= i + 1
        }
    }
    println!("Part 2 result: {}", c);
}
