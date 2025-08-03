
fn main() {
    let c1: char = '🦀';
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}


/*
fn main() {
    assert!(true && false == false);
    println!("Success: (true && false == false)");

    assert!(true || false == true);
    println!("Success: (true || false == true)");

    assert!(!true == false);
    println!("Success: (!true == false)");

    println!("0011 AND 0101 == {:04b} // Only 1 if both bits are 1", 0b0011 as u32 & 0b0101);        // 0001
    println!("0011 OR 0101 == {:04b} // 1 as long as one bit is 1", 0b0011 as u32 | 0b0101);         // 0111
    println!("0011 XOR 0101 == {:04b} // Only 1 if both bits are opposite", 0b0011 as u32 ^ 0b0101); // 0110
    println!("1 << 5 is {:04b} ({})", 1u32 << 5, 1u32 << 5); // 1 << 5 is 100000 (32)
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // 0x80 >> 2 is 0x20
}
*/

/*
fn main() {
    assert!(1u32 + 2 == 3);
    println!("Success: (1u32 + 2 == 3)");
    
    assert!(1i32 - 2 == -1);
    println!("Success: (1i32 - 2 == -1)");
    
    assert!(1i8 - 2 == -1);
    println!("Success: (1i8 - 2 == -1)");
    
    assert!(3 * 50 == 150);
    println!("Success: (3 * 50 == 150)");
    
    assert!(9.6f32 / 3.2f32 == 3.0); // Type f64 results in (9.6 / 3.2 == 2.9999999999999996)
    println!("Success: (9.6f32 / 3.2f32 == 3.0)");
    
    assert!(24 % 5 == 4);
    println!("Success: (24 % 5 == 4)");
}
*/

/*
use std::ops::{Range, RangeInclusive};
fn main() {
   assert_eq!((1..5), Range{start: 1, end: 5});
   println!("Success:");
   assert_eq!((1..=5), RangeInclusive::new(1, 5));
   println!("Success:");
}
*/

/*
fn main() {
    for c in 'a'..='z' {
        print!("{} ", c);
    }
    println!("");
    for c in 'a'..='z' { // a b c ...
        print!("{} ", c as u8); // covert letters to ASCII numbers: 97 98 99 ...
    }
    println!("");
}
*/

/*
fn main() {
    // assert!(0.1 + 0.2 == 0.3); // Panics because type f64 results in 0.30000000000000004 when adding 0.1 + 0.2
    // println!("{}", 0.1 + 0.2);
    
    // Convert to f32 to fix:
    assert!(0.1_f32 + 0.2_f32 == 0.3); // or assert!(0.1 as f32 + 0.2 as f32 == 0.3);
    println!("Success:");
}
*/

/*
fn main() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));
    println!("success:");
    
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    println!("success:");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
*/


/* 
fn main() {
    let s = String::from("hello");

    takes_ownership(&s);
    
    println!("{s}");

    // let i = 5;

    // makes_copy(i);

    // println!("{i}");

}

fn takes_ownership(some_string: &String) {
    println!("fn: {some_string}");
}

// fn makes_copy(some_integer: i32) {
//     println!("fn print: {some_integer}");
// }
 */


