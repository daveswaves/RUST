// src/tests/integration_test.rs
use rust_tests::greet;
use rust_tests::i8_max;

#[test]
fn test_greet() {
    let result = greet("Alice");
    assert_eq!(result, "Hello, Alice!");
}

#[test]
fn test_i8_max() {
    let result = i8_max();
    let test_ans = 126;
    assert_eq!(result, test_ans);
    println!("i8::MAX equals {}", test_ans);
}
