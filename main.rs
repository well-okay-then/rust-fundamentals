use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let n: i64 = iter.next().unwrap().unwrap().trim().parse().unwrap();

    println!("{}", square(n));
}

fn square(n: i64) -> i64 {
    n * n
}
