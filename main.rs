enum Light {
    Red,
    Yellow,
    Green,
}

fn next(l: Light) -> Light {
    // match on l
    match l {
        Light::Green => Light::Yellow,
        Light::Yellow => Light::Red,
        Light::Red => Light::Green,
    }
}

fn name(l: &Light) -> &str {
    match l {
        Light::Red => "red",
        Light::Yellow => "yellow",
        Light::Green => "green",
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let current = match input.trim() {
        "red" => Light::Red,
        "yellow" => Light::Yellow,
        _ => Light::Green,
    };
    println!("{}", name(&next(current)));
}
