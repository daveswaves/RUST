/* fn main() {
    // CODE_HERE
} */




/*
//=================================================================
// Using structs and debug display format
//=================================================================
#[derive(Debug)] // Required to display debug format below {rect1:?}
struct Rectangle {
    width: u32,
    height: u32,
}

// area method implementation block
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Display format doesn't work: println!("rect1 is {}", rect1);
    // But the debug format does:
    println!("rect1 is {rect1:?}"); // Using {rect1:#?} is like pretty print (indentation / separate line)
    println!("Area of rectangle ({} x {}): {}", &rect1.width, &rect1.height, area(&rect1));
    // Uses the area method above - much cleaner.
    println!("Area of rectangle ({} x {}): {}", &rect1.width, &rect1.height, rect1.area());
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/

/*
//=================================================================
// Using for loops
//=================================================================
fn main() {
    let arr = [10,20,30,40,50];

    for num in arr.iter() {
        println!("{}", num);
    }
    
    for num in 1..=4 {
        println!("{}", num);
    }
}
*/

/*
//=================================================================
// Extracting words from a string
//=================================================================
fn main() {
    let s = String::from("Hello world");
    let word1 = &s[0..=4];
    let word2 = &s[6..=10];
    
    println!("{} {}", word2, word1);
}
*/

/*
//=================================================================
// Using slice
//=================================================================
fn main() {
    let arr = [1,3,5,7,9];
    let slice1 = &arr[0..=1]; // [1,3]
    let slice2 = &arr[2..=4]; // [5,7,9]
    println!("{:?} {:?}", slice1, slice2);
}
*/
