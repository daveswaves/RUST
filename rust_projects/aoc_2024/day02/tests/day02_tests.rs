use day02::*;

// Run: cargo test --test day02_tests

const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

#[test]
// #[ignore]
fn day2_p1() {
    let result = day2(INPUT, false);
    assert_eq!(result, 2);
} 
