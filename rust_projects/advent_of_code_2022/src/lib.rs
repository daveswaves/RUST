use std::{
    cmp::Ordering,
    collections::HashMap,
    ops::RangeInclusive,
    str::FromStr,
};

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{newline, u32},
    multi::separated_list1,
};

// use nom:: Parser;

pub fn aoc01_p1(input: &str) -> String {
    let result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines() // better performance than split("\n")
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    result.to_string()
}

pub fn aoc01_p2(input: &str) -> String {
    let mut result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines() // better performance than split("\n")
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    result.sort_by( |a, b| b.cmp(a)); // reverse sort
    let sum: u32 = result.iter().take(3).sum();
    // dbg!(result);
    sum.to_string()
}

//-------------------------------

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

// https://youtu.be/rpG3ItDRgAg?list=PLWtPciJ1UMuBNTifxm5ADY65SkAdwoQiL&t=536
pub fn aoc02_p1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
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
            /*
            # The previous match block does the following:
            moves[1] as u32 + match (moves[0], moves[1]) {
                (Move::Rock, Move::Rock) => 3,
                (Move::Rock, Move::Paper) => 6,
                (Move::Rock, Move::Scissors) => 0,
                (Move::Paper, Move::Rock) => 0,
                (Move::Paper, Move::Paper) => 3,
                (Move::Paper, Move::Scissors) => 6,
                (Move::Scissors, Move::Rock) => 6,
                (Move::Scissors, Move::Paper) => 0,
                (Move::Scissors, Move::Scissors) => 3,
            }
            */
        }).sum();

    result.to_string()
}

// https://youtu.be/rpG3ItDRgAg?list=PLWtPciJ1UMuBNTifxm5ADY65SkAdwoQiL&t=752
pub fn aoc02_p2(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let (opponent_str, outcome_code) = line
                .split_once(' ')
                .expect("Invalid line format");

            let opponent_move = opponent_str.parse::<Move>().unwrap();

            // Determine our move and score modifier based on the strategy
            let (score_bonus, our_move) = match outcome_code {
                "X" => (0, match opponent_move {
                    Move::Rock => Move::Scissors,
                    Move::Paper => Move::Rock,
                    Move::Scissors => Move::Paper,
                }),
                "Y" => (3, opponent_move),
                "Z" => (6, match opponent_move {
                    Move::Rock => Move::Paper,
                    Move::Paper => Move::Scissors,
                    Move::Scissors => Move::Rock,
                }),
                _ => panic!("Unexpected response"),
            };

            score_bonus + our_move as u32
        })
        .sum();

    result.to_string()
}

//-------------------------------

// https://youtu.be/yBJJYkC5cdk?list=PLWtPciJ1UMuBNTifxm5ADY65SkAdwoQiL&t=167
pub fn aoc03_p1(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx+1))
        .collect::<HashMap<char, usize>>();
    // {'a': 1, 'b': 2, 'c': 3, ... 'X': 50, 'Y': 51, 'Z': 52}

    let result: usize = input
        .lines()
        .map(|line| {
            let sack_length = line.len() / 2;
            let compartment_a = &line[..sack_length];
            let compartment_b = &line[sack_length..];
            
            let common_char = compartment_a
                .chars()
                .find(|c| compartment_b.contains(*c))
                .unwrap();
            
            letter_scores.get(&common_char).unwrap()
    }).sum::<usize>();
    // Splits string into two halves: compartment_a | compartment_b.
    // Finds first character that appears in both (case sensitive).
    // Looks up score from letter_scores map and sums all scores.
    // vJrwpWtwJgWr hcsFMMfFFhFp > 'p' > 16
    // jqHRNqRjqzjGDLGL rsFMfFZSrLrFZsSL > 'L' > 38
    // PmmdzqPrV vPwwTWBwg > 'P' > 42
    // wMqvLMZHhHMvwLH jbvcjnnSBnvTQFn > 'v' > 22
    // ttgJtRGJ QctTZtZT > 't' > 20
    // CrZsJsPPZsGz wwsLwLmpwMDw > 's' > 19
    // 16 + 38 + 42 + 22 + 20 + 19 = 157

    result.to_string()
}

// https://youtu.be/yBJJYkC5cdk?list=PLWtPciJ1UMuBNTifxm5ADY65SkAdwoQiL&t=1302
pub fn aoc03_p2(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx+1))
        .collect::<HashMap<char, usize>>();
    
    use itertools::Itertools; // Must be listed under dependencies in Cargo.toml

    let result = input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(bag_a, bag_b, bag_c)| {
            let common_char = bag_a
                .chars()
                .find(|a_char| bag_b.contains(*a_char) && bag_c.contains(*a_char))
                .unwrap();

            letter_scores.get(&common_char).unwrap()
        })
        .sum::<usize>();

    result.to_string()
}

fn sections(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, start) = u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = u32(input)?;
    Ok((input, start..=end))
}

fn line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (input, start) = sections(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, end) = sections(input)?;
    Ok((input, (start, end)))
}

fn section_assignments(input: &str) -> IResult<&str,
    Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
        let (input, ranges) = separated_list1(newline, line)(input)?;
        Ok((input, ranges))
        // let parser = separated_list1::<_, _, nom::error::Error<_>, _>(newline, line);
        // let (input, ranges) = parser.parse(input)?;
        // Ok((input, ranges))

        // let parser = separated_list1(newline, line);
        // let (input, ranges) = parser(input)?;
        // Ok((input, ranges))
        // let (input, ranges) = (separated_list1(newline, line))(input)?;
}

// https://youtu.be/Xm4jrjohDN8?list=PLWtPciJ1UMuBNTifxm5ADY65SkAdwoQiL&t=267
pub fn aoc04_p1(input: &str) -> String {
    let (_, assignments) = section_assignments(input).unwrap();
    dbg!(assignments);
    "result".to_string()
}
pub fn aoc04_p2(_input: &str) -> String {
    "result".to_string()
}

// Ctrl+Shift+P > Open Keyboard Shortcuts (JSON) > "settings.cycle" command (requires Settings Cycler extension)
// Ctrl+Shift+P > Open User Settings (JSON) > "rust-analyzer.inlayHints.chainingHints.enable": false, "rust-analyzer.inlayHints.typeHints.enable": false,

// dbg!();




#[cfg(test)]
mod testing {
    use super::*;

    const INPUT_AOC01: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn aoc01_p1_works() {
        let result = aoc01_p1(INPUT_AOC01);
        assert_eq!(result, "24000");
    }

    #[test]
    // #[ignore]
    fn aoc01_p2_works() {
        let result = aoc01_p2(INPUT_AOC01);
        assert_eq!(result, "45000");
    }

    static INPUT_AOC02: &str = "A Y\nB X\nC Z";

    #[test]
    fn aoc02_p1_works() {
        // Rock vs Paper -> Loss => 6 + 2
        // Paper vs Rock -> Win => 0 + 1
        // Scissors vs Scissors -> Draw => 3 + 3
        assert_eq!(aoc02_p1(INPUT_AOC02), "15")
    }
    
    #[test]
    // #[ignore]
    fn aoc02_p2_works() {
        assert_eq!(aoc02_p2(INPUT_AOC02), "12");
    }

    const INPUT_AOC03: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn aoc03_p1_works() {
        let result = aoc03_p1(INPUT_AOC03);
        assert_eq!(result, "157");
    }
    
    #[test]
    // #[ignore]
    fn aoc03_p2_works() {
        let result = aoc03_p2(INPUT_AOC03);
        assert_eq!(result, "70");
    }

    const INPUT_AOC04: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn aoc04_p1_works() {
        let result = aoc04_p1(INPUT_AOC04);
        assert_eq!(result, "2");
    }
    
    #[test]
    #[ignore]
    fn aoc04_p2_works() {
        let result = aoc04_p2(INPUT_AOC04);
        assert_eq!(result, "4");
    }

}