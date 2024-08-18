use core::str;

#[derive(Debug, Clone)]
struct Point {
    x: i8,
    y: i8,
}

#[derive(Debug)]
struct Ship {
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
    
    fn get_rotated(&self) -> Ship {
        let mut new_shape = Vec::new();
        for p in &self.shape {
            new_shape.push(Point { x: -p.y, y: p.x });
        }
        Ship {
            pos: Point { x: self.pos.x, y: self.pos.y },
            shape: new_shape,
        }
    }

    fn bounds(&self) -> (i8, i8, i8, i8) {
        let mut minx = 127;
        let mut miny = 127;
        let mut maxx = -127;
        let mut maxy = -127;
        for p in &self.shape {
            if p.x < minx {
                minx = p.x;
            }
            if p.y < miny {
                miny = p.y;
            }
            if p.x > maxx {
                maxx = p.x;
            }
            if p.y > maxy {
                maxy = p.y;
            }
        }
        return (minx, miny, maxx, maxy);
    }

    fn intersects(&self, other: &Ship) -> bool {
        for p in &self.shape {
            for p2 in &other.shape {
                if p.x + self.pos.x == p2.x + other.pos.x && p.y + self.pos.y == p2.y + other.pos.y {
                    return true;
                }
            }
        }
        return false;
    }

    fn intersects_any(&self, others: &Vec<Ship>) -> bool {
        for other in others {
            if self.intersects(other) {
                return true;
            }
        }
        return false;
    }

    fn get_moved(&self, x: i8, y: i8) -> Ship {
        Ship {
            pos: Point { x: self.pos.x + x, y: self.pos.y + y },
            shape: self.shape.clone(),
        }
    }

    fn clone(&self) -> Ship {
        let new_pos = Point {
            x: self.pos.x,
            y: self.pos.y,
        };
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

const _BOLD: &str = "\u{001B}[1m";
const _BLACK: &str = "\u{001B}[30m";
const _RED: &str = "\u{001B}[31m";
const _GREEN: &str = "\u{001B}[32m";
const _YELLOW: &str = "\u{001B}[x33m";
const _BLUE: &str = "\u{001B}[34m";
const _PURPLE: &str = "\u{001B}[35m";
const _CYAN: &str = "\u{001B}[36m";
const _WHITE: &str = "\u{001B}[37m";
const _GRAY: &str = "\u{001B}[90m";
const _RESET: &str = "\x1B[0m";

const CROSSHAIRCOL: &str = _GREEN;

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
        shape: vec![Point { x: 0, y: 0 }, Point { x: 0, y: 1 }],
    };

    let mut sss = Vec::new();
    sss.push(squigly.clone());
    sss.push(strait4.clone());
    sss.push(straight3.clone());
    sss.push(straight2.clone());
    let mut ships: Vec<(Ship, Vec<String>)> = Vec::new();
    let mut ships_p2: Vec<(Ship, Vec<String>)> = Vec::new();
    let mut downed_ships: Vec<Ship> = Vec::new();
    let mut downed_ships_p2: Vec<Ship> = Vec::new();

    let mut renderships: Vec<Vec<String>> = Vec::new();

    for ship in &sss {
        let mut rship: Vec<String> = Vec::new();
        for y in 0..4 {
            let mut r = String::new();
            for x in 0..2 {
                if ship.shape.iter().any(|p| p.x == x && p.y == y) {
                    print!("■ ");
                    r.push_str("■ ");
                } else {
                    print!("□ ");
                    r.push_str("  ");
                }
            }
            if !r.replace(" ", "").is_empty() {
                rship.push(r);
            }
            println!();
        }
        renderships.push(rship);
        println!();
    }
    ships.push((squigly.clone(), renderships[0].clone()));
    ships.push((strait4.clone(), renderships[1].clone()));
    ships.push((straight3.clone(), renderships[2].clone()));
    ships.push((straight2.clone(), renderships[3].clone()));

    ships_p2.push((squigly, renderships[0].clone()));
    ships_p2.push((strait4, renderships[1].clone()));
    ships_p2.push((straight3, renderships[2].clone()));
    ships_p2.push((straight2, renderships[3].clone()));

    let mut selec_ship = 0;
    loop {
        clearscreen::clear().unwrap();

        print!("{_RED}{_BOLD}P1{_RESET}│ ");
        for i in 1..WIDTH + 1 {
            print!("{i} ", i = i);
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
                    if s.0
                        .shape
                        .iter()
                        .any(|p| p.x == x - s.0.pos.x && p.y == y - s.0.pos.y)
                    {
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

        //* Get user input
        println!("\n{_CYAN}{_BOLD}W{_RESET} Up      {_CYAN}{_BOLD}S{_RESET} Down\n{_CYAN}{_BOLD}A{_RESET} Left    {_CYAN}{_BOLD}D{_RESET} Right\n{_CYAN}{_BOLD}R{_RESET} Rotate  {_CYAN}{_BOLD}Q{_RESET} Switch ship\n       {_CYAN}{_BOLD}E{_RESET} Done\n", );
        match input_move() {
            Move::Up => {
                if ships[selec_ship].0.bounds().1 + ships[selec_ship].0.pos.y > 0 /* && ships[selec_ship].0.get_moved(0, -1).intersects_any() == false */ {
                    ships[selec_ship].0.pos.y -= 1;
                }
            }
            Move::Down => {
                if ships[selec_ship].0.bounds().3 + ships[selec_ship].0.pos.y < HEIGHT - 1 {
                    ships[selec_ship].0.pos.y += 1;
                }
            }
            Move::Left => {
                println!("{:?}", ships[selec_ship].0.bounds());
                if ships[selec_ship].0.bounds().0 + ships[selec_ship].0.pos.x > 0 {
                    ships[selec_ship].0.pos.x -= 1;
                }
            }
            Move::Right => {
                println!("{:?}", ships[selec_ship].0.bounds());
                if ships[selec_ship].0.bounds().2 + ships[selec_ship].0.pos.x < WIDTH - 1 {
                    ships[selec_ship].0.pos.x += 1;
                }
            }
            Move::Rotate => {
                let rotaded: Ship = ships[selec_ship].0.clone().get_rotated();
                if rotaded.bounds().0 + rotaded.pos.x >= 0
                    && rotaded.bounds().2 + rotaded.pos.x < WIDTH
                    && rotaded.bounds().1 + rotaded.pos.y >= 0
                    && rotaded.bounds().3 + rotaded.pos.y < HEIGHT
                {
                    ships[selec_ship].0.rotate();
                }
            }
            Move::Switch => {
                selec_ship = (selec_ship + 1) % ships.len();
            }
            Move::Done => {
                break;
            }
        }
    }

    clearscreen::clear().unwrap();

    println!("Player 2's turn to place ships");

    input();

    let mut selec_ship = 0;
    loop {
        let _ = clearscreen::clear();

        print!("{_BLUE}{_BOLD}P2{_RESET}│ ");
        for i in 1..WIDTH + 1 {
            print!("{i} ", i = i);
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
                for s in &ships_p2 {
                    if s.0
                        .shape
                        .iter()
                        .any(|p| p.x == x - s.0.pos.x && p.y == y - s.0.pos.y)
                    {
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

        //* Get user input
        println!("\n{_CYAN}{_BOLD}W{_RESET} Up      {_CYAN}{_BOLD}S{_RESET} Down\n{_CYAN}{_BOLD}A{_RESET} Left    {_CYAN}{_BOLD}D{_RESET} Right\n{_CYAN}{_BOLD}R{_RESET} Rotate  {_CYAN}{_BOLD}Q{_RESET} Switch ship\n       {_CYAN}{_BOLD}E{_RESET} Done\n", );
        match input_move() {
            Move::Up => {
                if ships_p2[selec_ship].0.pos.y + ships_p2[selec_ship].0.bounds().1 > 0 {
                    ships_p2[selec_ship].0.pos.y -= 1;
                }
            }
            Move::Down => {
                if ships_p2[selec_ship].0.pos.y + ships_p2[selec_ship].0.bounds().3 < HEIGHT - 1 {
                    ships_p2[selec_ship].0.pos.y += 1;
                }
            }
            Move::Left => {
                if ships_p2[selec_ship].0.pos.x + ships_p2[selec_ship].0.bounds().0 > 0 {
                    ships_p2[selec_ship].0.pos.x -= 1;
                }
            }
            Move::Right => {
                if ships_p2[selec_ship].0.pos.x + ships_p2[selec_ship].0.bounds().2 < WIDTH - 1 {
                    ships_p2[selec_ship].0.pos.x += 1;
                }
            }
            Move::Rotate => {
                let rotaded: Ship = ships_p2[selec_ship].0.clone().get_rotated();
                if rotaded.bounds().0 + rotaded.pos.x >= 0
                    && rotaded.bounds().2 + rotaded.pos.x < WIDTH
                    && rotaded.bounds().1 + rotaded.pos.y >= 0
                    && rotaded.bounds().3 + rotaded.pos.y < HEIGHT
                {
                    ships_p2[selec_ship].0.rotate();
                }
            }
            Move::Switch => {
                selec_ship = (selec_ship + 1) % ships_p2.len();
            }
            Move::Done => {
                break;
            }
        }
    }

    let mut shots: Vec<Point> = Vec::new();
    let mut shots_p2: Vec<Point> = Vec::new();
    let mut iy: char = 'Ñ';
    let mut ix: i8 = 127;

    'game_loop: loop {
        let mut currship = 0;
        let mut currship2 = 2;
        let mut yoffset = 0;
        let mut yoffset2 = 0;
        let mut tempy = 0;
        let mut tempy2 = 0;

        if ships.len() == 0 {
            println!("All ships downed!, {_RED}{_BOLD}P2{_RESET} wins!");
            break;
        }

        if ships_p2.len() == 0 {
            println!("All ships downed!, {_RED}{_BOLD}P1{_RESET} wins!");
            break;
        }

        clearscreen::clear().unwrap();
        println!("{_RED}{_BOLD}Player 1's Turn");
        input();

        'p1: loop {
            let mut to_remove: Vec<usize> = Vec::new();
            // Check if a ship is downed
            for ship in &ships_p2 {
                let mut downed = true;
                for p in &ship.0.shape {
                    if !shots
                        .iter()
                        .any(|s| s.x == p.x + ship.0.pos.x && s.y == p.y + ship.0.pos.y)
                    {
                        downed = false;
                        break;
                    }
                }
                if downed {
                    downed_ships_p2.push(ship.0.clone());
                    to_remove.push(
                        ships_p2
                            .iter()
                            .position(|s| s.0.pos.x == ship.0.pos.x && s.0.pos.y == ship.0.pos.y)
                            .unwrap(),
                    );
                }
            }

            for i in to_remove.iter().rev() {
                ships_p2.remove(*i);
            }

            currship = 0;
            currship2 = 2;
            yoffset = 0;
            yoffset2 = 0;
            tempy = 0;
            tempy2 = 0;

            clearscreen::clear().unwrap();

            print!("{_RED}{_BOLD}P1{_RESET}│ ");
            for i in 1..WIDTH + 1 {
                print!(
                    "{}{}{i} ",
                    if i - 1 == ix { _BOLD } else { _RESET },
                    if i - 1 == ix { CROSSHAIRCOL } else { _RESET },
                    i = i
                );
            }

            println!(
                "│ Ships remaining: {_RED}{_BOLD}{num}{_RESET}",
                num = ships_p2.len()
            );
            print!("──┼─");
            for i in 1..WIDTH + 1 {
                print!("{}──", if i - 1 == ix { _BOLD } else { _RESET });
            }
            println!("┼────────────────────");

            for y in 0..HEIGHT {
                print!(
                    "{}{}{num} │ ",
                    if y == letter_to_number(iy) {
                        CROSSHAIRCOL
                    } else {
                        _RESET
                    },
                    if y == letter_to_number(iy) {
                        _BOLD
                    } else {
                        _RESET
                    },
                    num = number_to_letter(y)
                );
                'hor: for x in 0..WIDTH {
                    for ship in &downed_ships_p2 {
                        if ship
                            .shape
                            .iter()
                            .any(|p| p.x == x - ship.pos.x && p.y == y - ship.pos.y)
                        {
                            print!("{_GRAY}■{_RESET} ");
                            continue 'hor;
                        }
                    }
                    if shots.iter().any(|p| p.x == x && p.y == y) {
                        for ship in &ships_p2 {
                            if ship
                                .0
                                .shape
                                .iter()
                                .any(|p| p.x == x - ship.0.pos.x && p.y == y - ship.0.pos.y)
                            {
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
                    } else if letter_to_number(iy) == y {
                        print!("{CROSSHAIRCOL}══{_RESET}");
                        continue;
                    } else if ix == x {
                        print!("{CROSSHAIRCOL}║ {_RESET}");
                        continue;
                    } else {
                        print!("{_CYAN}■ {_RESET}");
                    }
                }
                print!("\x08 │  ");

                //* Render ships
                if currship < ships_p2.len() {
                    if y - yoffset < ships_p2[currship].1.len() as i8 {
                        print!("{}", ships_p2[currship].1[(y - yoffset) as usize]);
                        tempy += 1;
                    } else {
                        currship += 1;
                        yoffset = tempy + 1; // +1 because we want to skip the next line
                        tempy = 0;
                    }
                }

                print!("   ");

                if currship2 < ships_p2.len() {
                    if y - yoffset2 < ships_p2[currship2].1.len() as i8 {
                        print!("{}", ships_p2[currship2].1[(y - yoffset2) as usize]);
                        tempy2 += 1;
                    } else {
                        currship2 += 1;
                        yoffset2 = tempy2 + 1; // +1 because we want to skip the next line
                        tempy2 = 0;
                    }
                }

                print!("   ");
                println!();
            }

            //* Get user input
            println!("\n Enter a position to shoot at (A-H, 1-8) or ! to shoot: {_CYAN}{_BOLD}",);
            match input_any() {
                Position::Letter(letter) => {
                    iy = letter;
                }
                Position::Number(number) => {
                    ix = number - 1;
                }
                Position::Shoot(shoot) => {
                    if shoot {
                        shots.push(Point {
                            x: ix,
                            y: letter_to_number(iy),
                        });
                        // check if hit
                        let mut hit = Point {
                            x: ix,
                            y: letter_to_number(iy),
                        };
                        let mut hit_ship = false;
                        for ship in &ships_p2 {
                            if ship
                                .0
                                .shape
                                .iter()
                                .any(|p| p.x == hit.x - ship.0.pos.x && p.y == hit.y - ship.0.pos.y)
                            {
                                hit_ship = true;
                                break;
                            }
                        }
                        ix = 127;
                        iy = 'Ñ';
                        if !hit_ship {
                            break 'p1;
                        }
                    }
                }
            }
        }

        clearscreen::clear().unwrap();

        println!("Player 2's turn");

        input();

        'p2: loop {
            let mut to_remove: Vec<usize> = Vec::new();
            // Check if a ship is downed
            for ship in &ships {
                let mut downed = true;
                for p in &ship.0.shape {
                    if !shots_p2
                        .iter()
                        .any(|s| s.x == p.x + ship.0.pos.x && s.y == p.y + ship.0.pos.y)
                    {
                        downed = false;
                        break;
                    }
                }
                if downed {
                    downed_ships.push(ship.0.clone());
                    to_remove.push(
                        ships
                            .iter()
                            .position(|s| s.0.pos.x == ship.0.pos.x && s.0.pos.y == ship.0.pos.y)
                            .unwrap(),
                    );
                }
            }

            for i in to_remove.iter().rev() {
                ships.remove(*i);
            }


            currship = 0;
            currship2 = 2;
            yoffset = 0;
            yoffset2 = 0;
            tempy = 0;
            tempy2 = 0;

            clearscreen::clear().unwrap();

            print!("{_BLUE}{_BOLD}P2{_RESET}│ ");
            for i in 1..WIDTH + 1 {
                print!(
                    "{}{}{i} ",
                    if i - 1 == ix { _BOLD } else { _RESET },
                    if i - 1 == ix { CROSSHAIRCOL } else { _RESET },
                    i = i
                );
            }

            println!(
                "│ Ships remaining: {_RED}{_BOLD}{num}{_RESET}",
                num = ships.len()
            );
            print!("──┼─");
            for i in 1..WIDTH + 1 {
                print!("{}──", if i - 1 == ix { _BOLD } else { _RESET });
            }
            println!("┼────────────────────");

            for y in 0..HEIGHT {
                print!(
                    "{}{}{num} │ ",
                    if y == letter_to_number(iy) {
                        CROSSHAIRCOL
                    } else {
                        _RESET
                    },
                    if y == letter_to_number(iy) {
                        _BOLD
                    } else {
                        _RESET
                    },
                    num = number_to_letter(y)
                );
                'hor: for x in 0..WIDTH {
                    for ship in &downed_ships {
                        if ship
                            .shape
                            .iter()
                            .any(|p| p.x == x - ship.pos.x && p.y == y - ship.pos.y)
                        {
                            print!("{_GRAY}■{_RESET} ");
                            continue 'hor;
                        }
                    }
                    if shots_p2.iter().any(|p| p.x == x && p.y == y) {
                        for ship in &ships {
                            if ship
                                .0
                                .shape
                                .iter()
                                .any(|p| p.x == x - ship.0.pos.x && p.y == y - ship.0.pos.y)
                            {
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
                    } else if letter_to_number(iy) == y {
                        print!("{CROSSHAIRCOL}══{_RESET}");
                        continue;
                    } else if ix == x {
                        print!("{CROSSHAIRCOL}║ {_RESET}");
                        continue;
                    } else {
                        print!("{_CYAN}■ {_RESET}");
                    }
                }
                print!("\x08 │  ");

                //* Render ships
                if currship < ships.len() {
                    if y - yoffset < ships[currship].1.len() as i8 {
                        print!("{}", ships[currship].1[(y - yoffset) as usize]);
                        tempy += 1;
                    } else {
                        currship += 1;
                        yoffset = tempy + 1; // +1 because we want to skip the next line
                        tempy = 0;
                    }
                }

                print!("   ");

                if currship2 < ships.len() {
                    if y - yoffset2 < ships[currship2].1.len() as i8 {
                        print!("{}", ships[currship2].1[(y - yoffset2) as usize]);
                        tempy2 += 1;
                    } else {
                        currship2 += 1;
                        yoffset2 = tempy2 + 1; // +1 because we want to skip the next line
                        tempy2 = 0;
                    }
                }

                print!("   ");
                println!();
            }

            //* Get user input
            println!("\n Enter a position to shoot at (A-H, 1-8) or ! to shoot: {_CYAN}{_BOLD}",);
            match input_any() {
                Position::Letter(letter) => {
                    iy = letter;
                }
                Position::Number(number) => {
                    ix = number - 1;
                }
                Position::Shoot(shoot) => {
                    if shoot {
                        shots_p2.push(Point {
                            x: ix,
                            y: letter_to_number(iy),
                        });
                        // check if hit
                        let mut hit = Point {
                            x: ix,
                            y: letter_to_number(iy),
                        };
                        let mut hit_ship = false;
                        for ship in &ships {
                            if ship
                                .0
                                .shape
                                .iter()
                                .any(|p| p.x == hit.x - ship.0.pos.x && p.y == hit.y - ship.0.pos.y)
                            {
                                hit_ship = true;
                                break;
                            }
                        }
                        ix = 127;
                        iy = 'Ñ';
                        if !hit_ship {
                            break 'p2;
                        }
                    }
                }
            }
        }
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
    let binding = input().to_ascii_uppercase();
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
