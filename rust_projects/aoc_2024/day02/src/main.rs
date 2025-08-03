use day02::*;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./day2.txt").unwrap();
    // let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    let part2 = true;
    println!("Total (part 1): {}", day2(&input, part2)); // result: 639 | 674
}
