use std::env;

pub enum Test<'a> {
    One(&'a str),
    Two(&'a str),
    Both(&'a str),
}

pub fn parse_args() -> Test<'static> {
    let args: Vec<String> = env::args().collect();
    let text = if args.len() > 2 {
        if args[2].eq("example") {
            include_str!("../example.txt")
        } else {
            panic!("second argument can only be 'example'")
        }
    } else {
        include_str!("../input.txt")
    };
    if args.len() < 2 {
        Test::Both(text)
    } else {
        match &args[1].parse() {
            Ok(1) => Test::One(text),
            Ok(2) => Test::Two(text),
            Ok(0) => Test::Both(text),
            _ => panic!("invalid test number"),
        }
    }
}
