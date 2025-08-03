fn main() {
    let vec = vec![1, 2, 3];
    
    // let iter = &vec;
    let iter = vec.iter();
    // let iter = vec.into_iter();

    for num in iter {
        println!("{}", num);
    } 
    
    // let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
    // test(&input);
    // println!("Total: {}", day1_part1(&input));
}


#[allow(dead_code)]
fn day1_part1(input: &str) -> u32 {
    let pairs: Vec<(u32, u32)> = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            match (parts.next(), parts.next()) {
                (Some(a), Some(b)) => {
                    Some((a.parse::<u32>().ok()?, b.parse::<u32>().ok()?))
                }
                _ => None,
            }
        })
        .collect();

    let (mut left, mut right): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();

    left.sort();
    right.sort();

    left.iter()
        .zip(&right)
        .map(|(l, r)| (*l as i32 - *r as i32).abs() as u32)
        .sum()
}