// use std::collections::HashMap;

use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };


    /*
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    scores.insert(String::from("Red"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(15);

    println!("{scores:?}"); // {"Red": 10, "Yellow": 50, "Blue": 25}


    // Count word occurrences in text.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
    */

    /*
    for chr in "Hello".chars() {
        println!("{chr}");
    }
    for byt in "Hello".bytes() {
        println!("{byt}");
    }
    */
    
    // let str = "initial contents";
    /* let mut str = "initial contents".to_string();
    println!("{str}");
    str = "new contents".to_string();
    println!("{str}"); */

    /* 
    let mut v = vec!["one", "two", "three", "four", "five"];
    v.push("six");
    v.extend(["seven", "eight", "nine", "ten"]);

    let indx = 9;
    let indx_plus1 = indx + 1;
    let item = v.get(indx);
    match item {
        Some(item) => println!("Element {indx_plus1} (index {indx}): {item}"),
        None => println!("Element {indx_plus1} (index {indx}) does not exist."),
    }
    */
}
