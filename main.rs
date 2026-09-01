use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let n = iter.next().unwrap().unwrap();

    println!("{}", count(&n));
}

fn count(s: &str) -> usize {
    s.len()
}
