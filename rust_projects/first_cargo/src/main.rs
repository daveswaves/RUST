use std::io::{self, Write};

fn main() {
    input_with_prompt();
}

fn input_with_prompt() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    print!("Enter 1st number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line!");
    
    let num1: i64 = input1.trim().parse().unwrap();
    
    print!("Enter 2nd number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line!");
    
    let num2: i64 = input2.trim().parse().unwrap();
    
    let ans = num1 + num2;
    
    println!("Sum of 2 numbers ({} + {}): {}", num1, num2, ans);
}