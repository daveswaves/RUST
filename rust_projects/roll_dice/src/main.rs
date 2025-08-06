use rand::Rng;

fn main() {
    let res = roll2dice();
    let dice_total: usize = [res.0, res.1].iter().sum();
    
    println!("{:?} : {}", res, dice_total);
}

fn roll2dice() -> (usize, usize) {
    (rand::thread_rng().gen_range(1..=6), rand::thread_rng().gen_range(1..=6))
}