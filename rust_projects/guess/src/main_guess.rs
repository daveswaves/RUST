use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};
use colored::*;

fn main() {
    let secret_no: u8 = gen_num();
    
    loop {
        let tmp = user_input("Enter Number: ");
        let guess: u8 = match tmp.trim().parse() {
            Ok(num) => num, // Stops program returning error if any non ints (unsigned) are entered.
            Err(_) => continue,
        };
        // let guess: u8 = tmp.trim().parse().expect("Please enter a valid number!");
        
        let ans = sort(secret_no, guess);
        
        if "Correct!" == ans {
            println!("{}", ans.green());
            break;
        }
        else {
            println!("{}", ans.red());
        }
    }
}

fn sort(secret_no: u8, guess: u8) -> &'static str {
    let responses = ["Higher!", "Lower!", "Correct!"];
    
    match guess.cmp(&secret_no) {
        Ordering::Less => return responses[0],
        Ordering::Greater => return responses[1],
        Ordering::Equal => return responses[2],
    }
}

fn gen_num() -> u8 {
    rand::thread_rng().gen_range(1..=100) // Requires rand = "0.8" in Cargo.toml
}

fn user_input(prompt: &str)->String {
    let mut input = String::new();
    
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    
    input
}
