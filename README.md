# aoc-2022

This is where I keep my answers for each day of [Advent of Code 2022](https://adventofcode.com/2022/)

Each folder is an independent Rust project. The program inputs are stored in example.txt for the samples given in the problem and input.txt for the full puzzle input.

Argument parsing is split into it's own file, parse_args.rs, for code clarity.

You can run each day with `cargo run`

You can specify which test to run with `cargo run [test num]`. Valid numbers are 1, 2, or 0 for both. 0 is the default.

You can test with the example input with `cargo run [test num] example`
