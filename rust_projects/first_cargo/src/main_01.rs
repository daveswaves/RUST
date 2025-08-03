

fn main() {
    // immutable_ints();
    // mutable_ints();
    // mutable_but_fixed_type();
    // scope();
    // constants();
    // get_var_types();
    // tuples();
    
}

// fn tuples() {
//     let tup: (i8, bool, char) = (1, true, 's');
//     println!("i8:{}, bool:{}, char:{}", tup.0, tup.1, tup.2); // i8:1, bool:true, char:s
// }


/*
fn get_var_types() {
    let num_int = 32;
    print_type_of(&num_int);       // i32
    
    let num_float = 32.90;
    print_type_of(&num_float);     // f64
    
    let str_var = "My String!";
    print_type_of(&str_var);       // &str
    
    let character = 'd';           // Note: Double quotes would result in &str
    print_type_of(&character);     // char
    
    let true_or_false = false;
    print_type_of(&true_or_false); // bool
    
    let arr = [1, 2, 3, 4, 5];
    print_type_of(&arr);           // [i32; 5]
}

fn print_type_of<T>(_: &T) {
    use std::any::type_name; // Can also be at top of file.
    println!("{}", type_name::<T>());
}
*/

/*
fn immutable_ints() {
    let x = 4;
    println!("{}", x);
    // x = 5; // cannot re-assign. Varaibles are immutable by default.
    
    /*
    Must use string literal: println!("{}", x);
    Not: println!(x);
    */
}

fn mutable_ints() {
    let mut x = 4;
    println!("{}", x);
    x = 5;
    println!("{}", x);
}

fn mutable_but_fixed_type() {
    let mut x = 4;
    println!("{}", x);
    // Rust assigned type as int, so cannot re-assign.
    // x = "Hello"; // expected integer, found `&str`
}

fn scope() {
    let  x = 4;
    println!("x is: {}", x); // x is: 4
    
    {
        let  x = x - 3;
        println!("x is: {}", x); // x is: 1
    }
    
    {
        let  x = 2;
        println!("x is: {}", x); // x is: 2
    }
    
    let x = x + 1;
    println!("x is: {}", x); // x is: 5
}

fn constants() {
    const SECONDS_IN_1HOUR: u16 = 3600;
    
    println!("SECONDS_IN_1HOUR constant is: {}", SECONDS_IN_1HOUR);
}
*/
/*
Types:
u8 : 0..255                   (0     .. 2^8-1)
i8 : -128..127                (-2^7  .. 2^7-1)
u16 : 0..65535                (0     .. 2^16-1)
i16 : -32768..32767           (-2^15 .. 2^15-1)
u32 : 0..4294967295           (0     .. 2^32-1)
i32 : -2147483648..2147483647 (-2^31 .. 2^31-1)
u64 : 0..18446744073709551615 (etc.)
i64 : -9223372036854775808..9223372036854775807
*/
