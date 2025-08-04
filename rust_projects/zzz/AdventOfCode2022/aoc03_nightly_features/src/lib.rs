pub fn process_part1(input: &str) -> String {
    let result = "one";
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
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
        assert_eq!(true, false);
    }
    
    #[test]
    fn part2_works() {
        assert_eq!(true, false);
    }
}
