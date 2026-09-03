use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let nums: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let answer = nums
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .sum::<i64>();

    println!("{}", answer);
}
