

fn main() {
    console_input()
    // console_input_with_prompt()
}

fn console_input() {
    use std::io;
    // let mut input = ""; doesn't work because that's type `&mut &str`. We need type `&mut String`.
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    println!("You entered: {}", input);
}

#[allow(dead_code)]
fn console_input_with_prompt() {
    use std::io::{self, Write};
    let mut input = String::new();
    
    print!("Enter: ");
    io::stdout().flush().expect("Failed to flush stdout");
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    
    println!("You entered: {}", input);
}
