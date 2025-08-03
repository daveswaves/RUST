# Create Rust Project

```
$ cargo new --lib day01
$ cd day01
$ touch src/main.rs
$ mkdir tests && touch tests/day01_tests.rs
```

```
day01/
   src/
      lib.rs
      main.rs
   tests/
      day01_tests.rs
   Cargo.toml
```

## Cargo.toml
```toml
[package]
name = "day01"
version = "0.1.0"
edition = "2024"

[dependencies]

[lib]
name = "day01"
path = "src/lib.rs"
```

## lib.rs
```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
```

**Note:**  
The commented out test above has been moved to a separate file in the tests directory.  
If the above tests module was being used, specific tests can be run by appending the test's function name:
```
$ cargo test it_works
```

## main.rs
```rust
use day01::*;

fn main() {
    let total = add(3, 2);
    println!("Total: {}", total);
}
```

## day01_tests.rs
```rust
use day01::add;

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
```

## Run
```
$ cargo run
$ cargo test --test day01_tests
```