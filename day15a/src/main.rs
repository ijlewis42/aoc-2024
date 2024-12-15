use std::io;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();


    let mut grid : Vec<Vec<char>> = Vec::new();
    let mut moves : Vec<char> = Vec::new();
    let mut robot_pos = (0, 0);
    
    // non-idiomatic approach to reading all the input
    let mut second_part = false;
    for (y, line) in lines.enumerate() {
        let line = line.unwrap();
        let line = line.chars().collect::<Vec<char>>();

        if line.len() == 0 {
            second_part = true;
            continue;
        }

        if !second_part {
            // add each line as a vector of chars
            grid.push(line.clone());

            for (x, c) in line.into_iter().enumerate() {
                if c == '@' {
                    robot_pos = (x, y);
                }
            }
        } else {
            moves.extend(line);       
        }        
    }

    // size of grid
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    for m in moves.iter() {
        let (dx, dy): (i32, i32) = match m {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            _ => (0, 1)
        };

        let (x, y) = robot_pos;
        let (x, y): (i32, i32) = (x as i32, y as i32);

        //println!("pos {x} {y}");
        //println!("dir: {dx} {dy}");


        let mut i = 1;
        if grid[(y + dy * i) as usize][(x + dx * i) as usize] == '#' {
            // don't move
        } else if grid[(y + dy * i) as usize][(x + dx * i) as usize] == '.' {
            // move 1
            grid[(y + dy * i) as usize][(x + dx * i) as usize] = '@';
            grid[y as usize][x as usize] = '.';
            robot_pos = ((x + dx) as usize, (y + dy) as usize);
        } else {
            let mut success = true;
            while grid[(y + dy * i) as usize][(x + dx * i) as usize] != '.'
            {
                if grid[(y + dy * i) as usize][(x + dx * i) as usize] == '#' {
                    success = false;
                    break;
                }
                //println!("found {}", grid[(y + dy * i) as usize][(x + dx * i) as usize]);
                i += 1;
            }
            if success {
                //println!("shuffling {i}");
    
                while i > 0 {
                    //println!(" - {} {}", (y + dy * i), (x + dx * i));
                    grid[(y + dy * i) as usize][(x + dx * i) as usize] = grid[(y + dy * (i - 1)) as usize][(x + dx * (i - 1)) as usize];
                    i -= 1;
                }
                grid[y as usize][x as usize] = '.';
                robot_pos = ((x + dx) as usize, (y + dy) as usize);
            }
        }



        /*for line in grid.clone() {
            println!("{:?}", line.into_iter().collect::<String>());
        }
        println!("");*/
    }

    let mut total = 0;

    for (y, line) in grid.clone().iter().enumerate() {
        for (x, c) in line.into_iter().enumerate() {
            if *c == 'O' {
                total += 100 * y + x;
            }
        }
    }

    println!("{total}");
}
