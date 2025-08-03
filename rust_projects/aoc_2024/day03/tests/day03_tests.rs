use day03::*;

// Run: cargo test --test day03_tests

const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

#[test]
// #[ignore]
fn day3_p1() {
    let result = day3_part1(INPUT);
    assert_eq!(result, 161);
} 
