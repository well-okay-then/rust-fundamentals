use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let nums: Vec<i64> = line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut best = std::i64::MIN;
    for n in nums {
        if n > best {
            best = n;
        }
    }

    println!("{}", best);
}
