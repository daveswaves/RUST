RUST 100-exercises


/*
https://rust-exercises.com/100-exercises/02_basic_calculator/02_variables.html
https://github.com/mainmatter/100-exercises-to-learn-rust/blob/main/exercises/02_basic_calculator/02_variables/src/lib.rs
*/

// The lines below, starting with `///`, are called **documentation comments**.
// They attach documentation to the item that follows them. In this case, the `speed` function.
// If you run `cargo doc --open` from this exercise's directory, Rust will generate
// HTML documentation from these comments and open it in your browser.

/// Given the start and end points of a journey, and the time it took to complete it,
/// calculate the average speed.
#[allow(dead_code)]
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: define a variable named `distance` with the right value to get tests to pass
    //  Do you need to annotate the type of `distance`? Why or why not?
    let distance = end - start;

    // Don't change the line below
    distance / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(speed(10, 30, 10), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(speed(10, 31, 10), 2);
    }
}

// #########################################################

/*
https://rust-exercises.com/100-exercises/02_basic_calculator/03_if_else
https://github.com/mainmatter/100-exercises-to-learn-rust/blob/main/exercises/02_basic_calculator/03_if_else/src/lib.rs
*/

/// Return `12` if `n` is even,
/// `13` if `n` is divisible by `3`,
/// `17` otherwise.
#[allow(dead_code)]
fn magic_number(n: u32) -> u32 {
    if n % 2 == 0 {
        return 12;
    } else if n % 3 == 0 {
        return 13;
    } else {
        return 17;
    }
}

#[cfg(test)]
mod tests {
    use crate::magic_number;

    #[test]
    fn one() {
        assert_eq!(magic_number(1), 17);
    }

    #[test]
    fn two() {
        assert_eq!(magic_number(2), 12);
    }

    #[test]
    fn six() {
        assert_eq!(magic_number(6), 12);
    }

    #[test]
    fn nine() {
        assert_eq!(magic_number(9), 13);
    }

    #[test]
    fn high() {
        assert_eq!(magic_number(233), 17);
    }
}

// #########################################################

/*
https://rust-exercises.com/100-exercises/02_basic_calculator/05_factorial
https://github.com/mainmatter/100-exercises-to-learn-rust/blob/main/exercises/02_basic_calculator/05_factorial/src/lib.rs

https://rust-exercises.com/100-exercises/02_basic_calculator/06_while
https://github.com/mainmatter/100-exercises-to-learn-rust/blob/main/exercises/02_basic_calculator/06_while/src/lib.rs

Define a function named `factorial` that, given a non-negative integer `n`,
returns `n!`, the factorial of `n`.

The factorial of `n` is defined as the product of all positive integers up to `n`.
For example, `5!` (read "five factorial") is `5 * 4 * 3 * 2 * 1`, which is `120`.
`0!` is defined to be `1`.

We expect `factorial(0)` to return `1`, `factorial(1)` to return `1`,
`factorial(2)` to return `2`, and so on.

Use only what you learned! No loops yet, so you'll have to use recursion!
*/

#[allow(dead_code)]
fn factorial(n: u32) -> u32 {
    // Using recursion    
    if n > 1 {
        n * factorial(n - 1)
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}


fn factorial(mut n: u32) -> u32 {
    // Using while loop
    let mut result = 1;
    while n > 1 {
        result *= n;
        n -= 1;
    }
    result
}

// Factorial using a for loop
fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in (1..=n).rev() { // also works without .rev()
        result *= i;
    }
    result
}

// #########################################################

/*
https://rust-exercises.com/100-exercises/03_ticket_v1/01_struct
https://github.com/mainmatter/100-exercises-to-learn-rust/blob/main/exercises/03_ticket_v1/01_struct/src/lib.rs
*/

// Define a struct named `Order` with the following fields:
// - `price`, an unsigned integer
// - `quantity`, an unsigned integer
//
// It should also have a method named `is_available` that returns a `true` if the quantity is
// greater than 0, otherwise `false`.

struct Order {
    price: u32,
    quantity: u32
}

impl Order {
    fn is_available(&self) -> bool {
        self.quantity > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}

