// src/main.rs

use rust_tests::greet; // 'rust_tests' is package name as defined in Cargo.toml
use rust_tests::i8_max; // 'rust_tests' is package name as defined in Cargo.toml

// cargo test --test integration_test
// cargo run

fn main() {
    let name = "World";
    println!("{}", greet(name));

    println!("{}", i8_max());
}
