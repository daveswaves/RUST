use day04::*;

// Run: cargo test --test day04_tests

#[test]
// #[ignore]
fn t1() {
    let result = highlight((0,4,0,1)); // →
    assert_eq!(result, "****ABCD**
**********
**********
**********
**********
**********
**********
**********
**********
**********");
}
