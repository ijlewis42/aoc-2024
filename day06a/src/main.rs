use std::io;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    let mut grid : Vec<Vec<char>> = Vec::new();

    let mut pos = (0i32, 0i32);
    let mut dir_index = 0;

    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    // non-idiomatic approach to reading all the input
    for (y, line) in lines.enumerate() {
        let line = line.unwrap();
        
        if let Some(x) = line.find("^") {
            pos = (x as i32, y as i32);
        }

        let line = line.chars().collect();

        // add each line as a vector of chars
        grid.push(line);
    }

    // size of grid
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut total = 0;

    'out:
    loop {
        let (x, y) = pos;
        if grid[y as usize][x as usize] != 'X' {
            total += 1;
        } 
        grid[y as usize][x as usize] = 'X';

        loop {
            let (dx, dy) = dirs[dir_index];
            let (nx, ny) = (x + dx, y + dy); // bleurgh

            pos = (nx, ny);

            if nx < 0 || nx >= width || ny < 0 || ny >= height {
                break 'out;
            }
    
            if grid[ny as usize][nx as usize] == '#' {
                dir_index = (dir_index + 1) % dirs.len();
            } else {
                break;
            }
        }

        /*for line in grid.clone() {
            println!("{}", line.iter().collect::<String>())
        }
        println!("");*/
    }


    println!("{}", total);
}
