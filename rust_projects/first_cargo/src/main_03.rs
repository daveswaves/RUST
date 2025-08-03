fn main() {
    // adding_ints();
    // dividing_ints();
    
    max_min();
}


fn max_min() {
    let min_i32 = i32::MIN;
    let max_i32 = i32::MAX;
    let max_u64 = u64::MAX;
    let max_u128 = u128::MAX;
    
    println!("MIN i32: {}", min_i32);
    println!("MAX i32: {}", max_i32);
    println!("MAX u64: {}", max_u64);
    println!("MAX u128: {}", max_u128);
}


#[allow(dead_code)]
fn adding_ints() {
    let a = 255_u8;
    let b = 1u8;
    let c = 120_000 as u32;
    
    // let ans = x + y; // 255 max, so fails - remember ans is also u8
    
    // Type cast : works without () but less clear
    let ans = (a as u32) + (b as u32) + c;
    
    println!("{} + {} + {} = {}", a, b, c, ans); // 255 + 1 + 120000 = 120256
}

#[allow(dead_code)]
fn dividing_ints() {
    let x: u8 = 255;
    let y: u8 = 10;
    
    let ans = x / y;
    println!("{} / {} = {}", x, y, ans); // 25
    
    let x: f32 = 255.0;
    let y: f32 = 10.0;
    
    let ans = x / y;
    println!("{} / {} = {}", x, y, ans); // 25.5
}

// println!("You entered: {}", input);
// #[allow(dead_code)]

