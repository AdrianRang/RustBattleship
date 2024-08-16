struct Point {
    x: i8,
    y: i8,
}

struct  Ship {
    pos: Point,
    shape: Vec<Point>,
}

impl Ship {
    fn rotate(&mut self) {
        for p in &mut self.shape {
            let x = p.x;
            p.x = -p.y;
            p.y = x;
        }
    }
}

fn main() {
    const WIDTH: i8 = 10;
    const HEIGHT: i8 = 10;

    let mut s = Ship {
        pos: Point { x: 2, y: 2 },
        shape: vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
        ],
    };

    for _ in 0..5 {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if s.shape.iter().any(|p| p.x == x-s.pos.x && p.y == y-s.pos.y) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        s.rotate();
        input();
    }
}


fn input() -> String {
    // https://www.tutorialspoint.com/rust/rust_input_output.htm
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line;
}