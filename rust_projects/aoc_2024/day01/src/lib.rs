/* pub fn day1_part1(input: &str) -> u32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        
        let left_num = numbers[0].parse::<u32>().unwrap();
        let right_num = numbers[1].parse::<u32>().unwrap();
        
        left.push(left_num);
        right.push(right_num);
    }

    left.sort();
    right.sort();

    let total_distance: u32 = left.iter()
        .zip(&right)
        .map(|(l, r)| (*l as i32 - *r as i32).abs() as u32)
        .sum();

    total_distance
} */

pub fn day1_part1(input: &str) -> u32 {
    let pairs: Vec<(u32, u32)> = input
        .lines()
        .filter_map(|line| { // line > "3 4" (1st), "4 3" (2nd), "2 5" (3rd), etc.
            let mut parts = line.split_whitespace();
            match (parts.next(), parts.next()) {
                (Some(a), Some(b)) => {
                    Some((a.parse::<u32>().ok()?, b.parse::<u32>().ok()?))
                }
                _ => None,
            }
        })
        .collect();

    let (mut left, mut right): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();

    left.sort();
    right.sort();

    left.iter()
        .zip(&right)
        .map(|(l, r)| (*l as i32 - *r as i32).abs() as u32)
        .sum()
}


pub fn day1_part2(input: &str) -> u32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        
        let left_num = numbers[0].parse::<u32>().unwrap();
        let right_num = numbers[1].parse::<u32>().unwrap();
        
        left.push(left_num);
        right.push(right_num);
    }

    let result = weighted_sum_from_occurrences(&left, &right);

    result
}

pub fn weighted_sum_from_occurrences(left: &[u32], right: &[u32]) -> u32 {
    left.iter()
        .map(|&l| {
            let count = right.iter().filter(|&&r| r == l).count() as u32;
            l * count
        })
        .sum()
}

/*
fn main() {
    let nums = "5 3 9 5 3 1 7 5";
    
    let total = nums.matches("5").count();
    
    println!("{}", total);
}
*/


/* #[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";

    #[test]
    fn day1_p1() {
        let result = day1_part1(INPUT);
        assert_eq!(result, 11);
    }
} */
