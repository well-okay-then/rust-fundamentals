struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance_sq(&self, other: &Point) -> i32 {
        let x_sq = (self.x - other.x).pow(2);
        let y_sq = (self.y - other.y).pow(2);

        x_sq + y_sq
    }
}

fn main() {
    let lines: Vec<i32> = (0..4)
        .map(|_| {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim().parse().unwrap()
        })
        .collect();
    let a = Point {
        x: lines[0],
        y: lines[1],
    };
    let b = Point {
        x: lines[2],
        y: lines[3],
    };
    println!("{}", a.distance_sq(&b));
}
