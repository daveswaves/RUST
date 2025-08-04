use aoc01_summing_nums::process_part1;
use std::fs;

// Create project: cargo new --lib aoc01_summing_nums
// cargo run --bin part1

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&file));
}

// https://youtu.be/bkvSRfgDG-E?list=PLWtPciJ1UMuBNTifxm5ADY65SkAdwoQiL&t=748
