use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, Eq, Clone)]
enum Value {
    Array(Vec<Value>),
    Num(u8),
}

// I also did this with serde_json, which was ~3ms faster faster, but I wanted to see if I could deal with the list
//  stacking logic in rust.

// re-written again using https://github.com/SvetlinZarev/advent-of-code/blob/main/2022/aoc-day-13/src/lib.rs
//  Learned about .take and realized their current list pointer doesn't have any borrow issues
//  Also moved to FromStr impl so you can just call str.parse::<Value>()
impl FromStr for Value {
    type Err = String;
    fn from_str(packet_def: &str) -> Result<Self, Self::Err> {
        let mut num: Option<u8> = None;
        let mut lists: Vec<_> = vec![];
        let mut currlist: Vec<Value> = vec![];
        for c in packet_def[1..packet_def.len() - 1].bytes() {
            match c {
                b'[' => {
                    lists.push(currlist);
                    currlist = vec![];
                }
                b']' => {
                    if let Some(n) = num.take() {
                        currlist.push(Value::Num(n))
                    }

                    let list_to_add = Value::Array(currlist);
                    currlist = lists.pop().unwrap();
                    currlist.push(list_to_add);
                }
                b',' => {
                    if let Some(n) = num.take() {
                        currlist.push(Value::Num(n));
                    }
                }
                l => {
                    if let Some(n) = num {
                        num = Some((n * 10) + l - b'0')
                    } else {
                        num = Some(l - b'0')
                    }
                }
            }
        }
        Ok(Value::Array(currlist))
    }
}

// re-written again using https://github.com/SvetlinZarev/advent-of-code/blob/main/2022/aoc-day-13/src/lib.rs
//  realized we're using lexical ordering, so we can take advantage of rust built-ins. an array of values
//  can use the same comparison methods as the values themselves.
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Num(n1), Value::Num(n2)) => n1.cmp(n2),
            (n1 @ Value::Num(_), Value::Array(n2)) => std::slice::from_ref(n1).cmp(n2.as_slice()),
            (Value::Array(n1), n2 @ Value::Num(_)) => n1.as_slice().cmp(std::slice::from_ref(n2)),
            (Value::Array(n1), Value::Array(n2)) => n1.cmp(n2),
        }
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
        let packet1: Value = packetdef1.parse().unwrap();
        let packet2: Value = packetdef2.parse().unwrap();

        // if packet1 < packet 2, the ordering is correct, add the index + 1 to the counter.
        match packet1.cmp(&packet2) {
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
    let marker1 = Value::Num(6);
    let marker2 = Value::Num(2);

    // Build vec of packets
    let mut packets: Vec<Value> = vec![marker1.clone(), marker2.clone()];
    for packetdef in str_input.lines() {
        if !packetdef.is_empty() {
            packets.push(packetdef.parse().unwrap());
        }
    }

    // Sort vector using packet comparison method
    packets.sort_unstable();

    // calculate index(marker1) * index(marker2)
    let mut c = 1;
    for (i, p) in packets.iter().enumerate() {
        if *p == marker1 || *p == marker2 {
            c *= i + 1
        }
    }
    println!("Part 2 result: {}", c);
}
