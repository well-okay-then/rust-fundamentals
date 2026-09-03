fn parse_int(s: &str) -> Result<i32, String> {
    // TODO: parse s as an i32 and return Ok(n) when that works.
    // When it does not, retrurn Err with the message "not a number".
    s.trim().parse::<i32>().map_err(|_| format!("not a number"))
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match parse_int(&input) {
        Ok(n) => println!("ok: {}", n),
        Err(e) => println!("error: {}", e),
    }
}
