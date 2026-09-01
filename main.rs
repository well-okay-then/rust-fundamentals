use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let n: i64 = iter.next().unwrap().unwrap().trim().parse().unwrap();

    let mut total: i64 = 0;
    for i in 1..=n {
        total += i;
    }
    println!("{}", total);
}
