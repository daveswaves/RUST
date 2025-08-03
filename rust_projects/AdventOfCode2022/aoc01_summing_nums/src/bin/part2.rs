use aoc01_summing_nums::process_part2;
use std::fs;

// cargo run --bin part2

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part2(&file));
}