use regex::Regex;

pub fn day3_part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|cap| {
            let x: u32 = cap[1].parse().unwrap();
            let y: u32 = cap[2].parse().unwrap();
            x * y
        })
        .sum()

    /* let mut total = 0;
    for cap in re.captures_iter(input) {
        let x: u32 = cap[1].parse().unwrap();
        let y: u32 = cap[2].parse().unwrap();
        
        total += x * y;
    }
    total */
}

// #[allow(dead_code)]
pub fn day3_test(input: &str) -> u32 {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();

    let prods: Vec<&str> = re.find_iter(input)
        .map(|m| m.as_str())
        .collect();

    dbg!(prods);
    10
}