use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let input: Vec<String> = line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut seen = HashSet::new();
    for word in input {
        seen.insert(word);
    }

    println!("{}", seen.len());
}
