use day01::*;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input_files/day1.txt").unwrap();

    println!("Total (part 1): {}", day1_part1(&input));
    println!("Total (part 2): {}", day1_part2(&input));
}
