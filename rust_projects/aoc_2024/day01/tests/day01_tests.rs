use day01::*;

// Run: cargo test --test day01_tests

const INPUT: &str = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";

#[test]
fn day1_p1() {
    let result = day1_part1(INPUT);
    assert_eq!(result, 11);
}
/*
3 4 -> 1 3 = 2
4 3 -> 2 3 = 1
2 5 -> 3 3 = 0
1 3 -> 3 4 = 1
3 9 -> 3 5 = 2
3 3 -> 4 9 = 5
Total: 11
*/

// https://youtu.be/e5YfD4NHlCI?t=2942
#[test]
// #[ignore]
fn day1_p2() {
    let result = day1_part2(INPUT);
    assert_eq!(result, 31);
} 
/*
3 4 -> 3*3 = 9
4 3 -> 4*1 = 4
2 5 -> 2*0 = 0
1 3 -> 1*0 = 0
3 9 -> 3*3 = 9
3 3 -> 3*3 = 9
Total: 31
*/
