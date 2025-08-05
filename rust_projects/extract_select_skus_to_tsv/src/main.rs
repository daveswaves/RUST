/* use std::fs::read_to_string;

fn main() {
    // println!("Hello, world!");
} */


use std::fs::read_to_string;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write, BufRead, BufReader, Result};

/* use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./ignored_skus.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
        }
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
} */


fn main() {
    // let map = file_to_hashmap("./ignored_skus.txt").unwrap();
    let map = match file_to_hashmap("./files/ignored_skus.txt") {
        Ok(m) => m,
        Err(_e) => {
            println!("Required file does not exist. Extract 'files.7z' to create required files.");
            return
        }
    };
    
    // if map.get("'13701A'").is_some() {
        //     println!("Remove: '13701A'"); // {:#?}
    // }
    
    let tsv = read_lines("./files/new_skus_2025.08.04.tsv");

    let new_list = tsv.iter()
        // .filter(|s|  map.get(*s).is_some())
        .filter(|line| map.keys().any(|key| line.contains(key)))
        .cloned()
        .collect::<Vec<String>>();

    write_lines_to_file(&new_list, "./files/ignored_matches.tsv").unwrap();
    // println!("{:#?}", new_list);
}

fn write_lines_to_file(lines: &[String], filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);


    writeln!(writer, "{}", "keyphrase\tsku\ttitle".to_string())?;
    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}

fn file_to_hashmap(filename: &str) -> Result<HashMap<String, i32>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut map = HashMap::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let key = line.trim().to_string();
        if !key.is_empty() {
            map.insert(key, 1);
        }
    }

    Ok(map)
}

#[allow(dead_code)]
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)  // make each slice into a string
        .collect()
}