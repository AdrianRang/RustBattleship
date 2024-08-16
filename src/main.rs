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
    const WIDTH: i8 = 8;
    const HEIGHT: i8 = 8;

    let mut s = Ship {
        pos: Point { x: 2, y: 2 },
        shape: vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
        ],
    };

    for _ in 0..1 {
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
    }

    println!();

    let mut shots: Vec<Point> = Vec::new();
    let mut iy:char = 'Ñ';
    let mut ix:i8 = 127;
    for _turn in 0..5 {
        // shots.push(Point { x: 0, y: 0 });
        // shots.push(Point { x: 3, y: 3 });

        if iy == 'Ñ' {
            iy = input_letter();
        } else {
            ix = input_number();
        }

        print!("  │ ");
        for i in 1..WIDTH+1 {
            print!("{i} ")
        }

        println!("│");
        print!("──│─");
        for _ in 1..WIDTH+1 {
            print!("──");
        }
        println!("│");

        for y in 0..HEIGHT {
            print!("{} ├─", number_to_letter(y));
            for x in 0..WIDTH {
                if letter_to_number(iy) == y && ix == x {
                    print!("▓▓");
                    continue;
                }
                if letter_to_number(iy) == y{
                    print!("░░");
                    continue;
                }
                if ix == x {
                    print!("░░");
                    continue;
                } 
                if shots.iter().any(|p| p.x == x || p.y == y) {
                    if s.shape.iter().any(|p| p.x == x-s.pos.x && p.y == y-s.pos.y) {
                        print!("─X─");
                    } else {
                        print!("-O-");
                    }
                } else {
                    print!("┼─");
                }
            }
            println!("│");
        }
        println!();
    }
    
    // println!("Shooting at {}, {}", hit.x, hit.y);
    // if s.shape.iter().any(|p| p.x == hit.x-s.pos.x && p.y == hit.y-s.pos.y) {
    //     println!("Hit!");
    // } else {
    //     println!("Miss!");
    // }
}


fn input() -> String {
    // https://www.tutorialspoint.com/rust/rust_input_output.htm
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line;
}

fn input_number() -> i8 {
    
    return input().trim().parse().unwrap();
}

fn input_letter() -> char {
    return input().trim().chars().next().unwrap();
}

fn letter_to_number(letter: char) -> i8 {
    return letter as i8 - 'A' as i8;
}

fn number_to_letter(number: i8) -> char {
    return (number as u8 + 'A' as u8) as char;
}