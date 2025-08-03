use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Move::*;
        use Ordering::*;

        match (self, other) {
            (a, b) if a == b => Some(Equal),
            (Rock, Scissors) => Some(Greater),
            (Scissors, Paper) => Some(Greater),
            (Paper, Rock) => Some(Greater),
            _ => Some(Less),
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Not a known move".to_string())
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let result: u32 = input.lines().map(|line| {
        let moves: Vec<Move> = line
            .split_whitespace()
            .map(|s| s.parse::<Move>().unwrap())
            .collect();
        
        match moves[0].partial_cmp(&moves[1]) {
            Some(Ordering::Equal) => {3 + moves[1] as u32}
            Some(Ordering::Less) => {6 + moves[1] as u32}
            Some(Ordering::Greater) => {0 + moves[1] as u32}
            None => {panic!("moves should be comparable")}
        }
    }).sum();

    result.to_string()
}

// https://youtu.be/rpG3ItDRgAg?list=PLWtPciJ1UMuBNTifxm5ADY65SkAdwoQiL&t=536
pub fn process_part2(_input: &str) -> String {
    let result = "two";
    result.to_string()
}

/*
Run tests:
cargo test --lib
*/

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn part1_works() {
        // Rock vs Paper -> Loss => 6 + 2
        // Paper vs Rock -> Win => 0 + 1
        // Scissors vs Scissors -> Draw => 3 + 3
        assert_eq!(process_part1(INPUT), "15")
    }
    
    #[test]
    fn part2_works() {
        assert_eq!(true, false);
    }
}
