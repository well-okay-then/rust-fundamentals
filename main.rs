use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let n: i32 = iter.next().unwrap().unwrap().trim().parse().unwrap();

    match (n % 3 == 0, n % 5 == 0) {
        (true, true) => println!("FizzBuzz"),
        (true, false) => println!("Fizz"),
        (false, true) => println!("Buzz"),
        (false, false) => println!("{}", n),
    }
}
