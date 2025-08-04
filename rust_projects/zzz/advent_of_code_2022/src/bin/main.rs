use advent_of_code_2022::*;
use std::fs;
use std::env; // access args

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin main <aocXX>");
        return;
    }

    // Run Tests: cargo test --lib

    match args[1].as_str() {
        // cargo run --bin main aoc01
        "aoc01" => {
            let file = fs::read_to_string("./input_files/aoc01.txt").unwrap();
            println!("{}", aoc01_p1(&file));
            println!("{}", aoc01_p2(&file));
        }
        // cargo run --bin main aoc02
        "aoc02" => {
            let file = fs::read_to_string("./input_files/aoc02.txt").unwrap();
            println!("{}", aoc02_p1(&file));
            println!("{}", aoc02_p2(&file));
        }
        // cargo run --bin main aoc03
        "aoc03" => {
            let file = fs::read_to_string("./input_files/aoc03.txt").unwrap();
            println!("{}", aoc03_p1(&file));
            println!("{}", aoc03_p2(&file));
        }
        // cargo run --bin main aoc04
        "aoc04" => {
            let file = fs::read_to_string("./input_files/aoc04.txt").unwrap();
            println!("{}", aoc04_p1(&file));
            println!("{}", aoc04_p2(&file));
        }
        _ => {
            eprintln!("Unknown argument");
        }
    }
}
