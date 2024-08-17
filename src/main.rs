use core::str;

#[derive(Debug)]
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

    fn clone(&self) -> Ship {
        let new_pos = Point { x: self.pos.x, y: self.pos.y };
        let mut new_shape = Vec::new();
        for p in &self.shape {
            new_shape.push(Point { x: p.x, y: p.y });
        }
        Ship {
            pos: new_pos,
            shape: new_shape,
        }
    }
}

enum Position {
    Letter(char),
    Number(i8),
    Shoot(bool),
}

enum Move {
    Up,
    Down,
    Left,
    Right,
    Rotate,
    Switch,
    Done,
}

const _BOLD:&str = "\u{001B}[1m";
const _BLACK:&str = "\u{001B}[30m";
const _RED:&str = "\u{001B}[31m";
const _GREEN:&str = "\u{001B}[32m";
const _YELLOW:&str = "\u{001B}[x33m";
const _BLUE:&str = "\u{001B}[34m";
const _PURPLE:&str = "\u{001B}[35m";
const _CYAN:&str = "\u{001B}[36m";
const _WHITE:&str = "\u{001B}[37m";
const _GRAY:&str = "\u{001B}[90m";
const _RESET:&str = "\x1B[0m";

const CROSSHAIRCOL:&str = _GREEN;

fn main() {
    const WIDTH: i8 = 8;
    const HEIGHT: i8 = 8;

    let squigly = Ship {
        pos: Point { x: 2, y: 0 },
        shape: vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
        ],
    };
    let strait4 = Ship {
        pos: Point { x: 0, y: 4 },
        shape: vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 0, y: 3 },
        ],
    };
    let straight3 = Ship {
        pos: Point { x: 3, y: 4 },
        shape: vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
        ],
    };
    let straight2 = Ship {
        pos: Point { x: 0, y: 0 },
        shape: vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
        ],
    };

    let mut ships: Vec<Ship> = Vec::new();
    let mut downed_ships: Vec<Ship> = Vec::new();

    ships.push(squigly);
    ships.push(strait4);
    ships.push(straight3);
    ships.push(straight2);

    // DEBUG Render ships
    for ship in &ships {
        for y in 0..4 {
            for x in 0..2 {
                if ship.shape.iter().any(|p| p.x == x && p.y == y) {
                    print!("■ ");
                } else {
                    print!("□ ");
                }
            }
            println!();
        }
        println!();
    }

    let mut selec_ship = 0;
    loop {

        
        match input_move() {
            Move::Up => {
                if ships[selec_ship].pos.y > 0 {
                    ships[selec_ship].pos.y -= 1;
                }
            },
            Move::Down => {
                if ships[selec_ship].pos.y < HEIGHT-1 {
                    ships[selec_ship].pos.y += 1;
                }
            },
            Move::Left => {
                if ships[selec_ship].pos.x > 0 {
                    ships[selec_ship].pos.x -= 1;
                }
            },
            Move::Right => {
                if ships[selec_ship].pos.x < WIDTH-1 {
                    ships[selec_ship].pos.x += 1;
                }
            },
            Move::Rotate => {
                ships[selec_ship].rotate();
            },
            Move::Switch => {
                selec_ship = (selec_ship + 1) % ships.len();
            },
            Move::Done => {
                break;
            },
        }

        let _ = clearscreen::clear();

        print!("  │ ");
        for i in 1..WIDTH+1 {
            print!("{i} ", i=i);
        }
        println!("|");
        print!("──┼─");
        for _ in 0..WIDTH {
            print!("──");
        }
        println!("┐");
        for y in 0..HEIGHT {
            print!("{} │ ", number_to_letter(y));
            for x in 0..WIDTH {
                let mut i = 0;
                let mut ship = false;
                for s in &ships {
                    if s.shape.iter().any(|p| p.x == x-s.pos.x && p.y == y-s.pos.y) {
                        ship = true;
                        break;
                    }
                    i += 1;
                }
                if ship {
                    print!("{}■ {_RESET}", if i == selec_ship { _CYAN } else { _RESET });
                } else {
                    print!("  ");
                }
            }
            println!("│");
        }
        print!("  └");
        for _ in 0..WIDTH {
            print!("──");
        }
        println!("─┘");
    }


    println!();

    let mut shots: Vec<Point> = Vec::new();
    let mut iy:char = 'Ñ';
    let mut ix:i8 = 127;
    loop {
        if ships.len() == 0 {
            println!("All ships downed!");
            break;
        }

        match input_any() {
            Position::Letter(letter) => {
                iy = letter;
            },
            Position::Number(number) => {
                ix = number - 1;
            },
            Position::Shoot(shoot) => {
                if shoot {
                    shots.push(Point { x: ix, y: letter_to_number(iy) });
                    ix = 127;
                    iy = 'Ñ';
                }
            }
        }

        let _ = clearscreen::clear();

        let mut to_remove: Vec<usize> = Vec::new();
        // Check if a ship is downed
        for ship in &ships {
            let mut downed = true;
            for p in &ship.shape {
                if !shots.iter().any(|s| s.x == p.x+ship.pos.x && s.y == p.y+ship.pos.y) {
                    downed = false;
                    break;
                }
            }
            if downed {
                downed_ships.push(ship.clone());
                to_remove.push(ships.iter().position(|s| s.pos.x == ship.pos.x && s.pos.y == ship.pos.y).unwrap());
            }
        }

        for i in to_remove.iter().rev() {
            ships.remove(*i);
        }

        print!("{_RED}{_BOLD}P1{_RESET}│ ");
        for i in 1..WIDTH+1 {
            print!("{}{}{i} ", if i-1 == ix { _BOLD } else { _RESET }, if i-1 == ix { CROSSHAIRCOL } else { _RESET }, i=i);
        }

        println!("│ Ships remaining: {_RED}{_BOLD}{num}{_RESET}", num=ships.len());
        print!("──┼─");
        for i in 1..WIDTH+1 {
            print!("{}──", if i-1 == ix { _BOLD } else { _RESET });
        }
        println!("┼────────────────────");

        for y in 0..HEIGHT {
            print!("{}{}{num} │ ", if y==letter_to_number(iy) { CROSSHAIRCOL } else { _RESET }, if y==letter_to_number(iy) { _BOLD } else { _RESET }, num=number_to_letter(y));
            'hor: for x in 0..WIDTH {
                for ship in &downed_ships {
                    if ship.shape.iter().any(|p| p.x == x-ship.pos.x && p.y == y-ship.pos.y) {
                        print!("{_GRAY}■{_RESET} ");
                        continue 'hor;
                    }
                }
                if shots.iter().any(|p| p.x == x && p.y == y) {
                    for ship in &ships {
                        if ship.shape.iter().any(|p| p.x == x-ship.pos.x && p.y == y-ship.pos.y) {
                            print!("{_RED}■{_RESET} ");
                            continue 'hor;
                        } else {
                            continue;
                        }
                    }
                    print!("{_RESET}■{_RESET} ");
                } else if letter_to_number(iy) == y && ix == x {
                    print!("{CROSSHAIRCOL}╬═{_RESET}");
                    continue;
                } else if letter_to_number(iy) == y{
                    print!("{CROSSHAIRCOL}══{_RESET}");
                    continue;
                } else if ix == x {
                    print!("{CROSSHAIRCOL}║ {_RESET}");
                    continue;
                } else {
                    print!("{_CYAN}■ {_RESET}");
                }
            }
            println!("\x08 │");
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

fn letter_to_number(letter: char) -> i8 {
    return letter as i8 - 'A' as i8;
}

fn number_to_letter(number: i8) -> char {
    return (number as u8 + 'A' as u8) as char;
}

fn input_any() -> Position {
    let binding = input();
    let input: &str = binding.trim();
    if input == "!" {
        Position::Shoot(true)
    } else if let Ok(number) = input.parse::<i8>() {
        Position::Number(number)
    } else if let Some(letter) = input.chars().next() {
        Position::Letter(letter)
    } else {
        panic!("Invalid input");
    }
}

fn input_move() -> Move {
    let binding = input();
    let input: &str = binding.trim();
    match input {
        "w" => Move::Up,
        "s" => Move::Down,
        "a" => Move::Left,
        "d" => Move::Right,
        "r" => Move::Rotate,
        "q" => Move::Switch,
        "e" => Move::Done,
        _ => panic!("Invalid input"),
    }
}