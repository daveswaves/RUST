use day03::*;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./day3.txt").unwrap();
    // let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    println!("Total (part 1): {}", day3_part1(&input)); // result: 
}
