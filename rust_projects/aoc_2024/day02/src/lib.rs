// Check that numbers on each line are all increasing or all decreasing
// and difference between adjacent pairs no greater than 3.
// #[allow(dead_code)]
/* pub fn day2_part1(input: &str) -> u32 {
    input.lines()
        .filter(|line| {
            let nums: Vec<u32> = line
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            let is_increasing = nums.windows(2).all(|w| w[0] < w[1] && (w[1] as i32 - w[0] as i32).abs() <= 3);
            let is_decreasing = nums.windows(2).all(|w| w[0] > w[1] && (w[1] as i32 - w[0] as i32).abs() <= 3);
            
            is_increasing || is_decreasing
        })
        .count() as u32
} */

pub fn day2(input: &str, part2: bool) -> u32 {
    input.lines()
        .filter(|line| {
            let nums: Vec<u32> = line
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            if is_valid(&nums) {
                true
            } else {
                if part2 {try_remove_one(&nums)}
                else {false}
            }
        })
        .count() as u32
}

fn is_valid(nums: &[u32]) -> bool {
    let is_increasing = nums.windows(2).all(|w| w[0] < w[1] && (w[1] as i32 - w[0] as i32).abs() <= 3);
    let is_decreasing = nums.windows(2).all(|w| w[0] > w[1] && (w[1] as i32 - w[0] as i32).abs() <= 3);
    is_increasing || is_decreasing
}

// #[allow(dead_code)]
fn try_remove_one(nums: &[u32]) -> bool {
    for i in 0..nums.len() {
        let mut reduced = nums.to_vec();
        reduced.remove(i);
        if is_valid(&reduced) {
            return true;
        }
    }
    false
}



// https://youtu.be/e5YfD4NHlCI?t=4541