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

    // this probably took half an hour to run lol
    for j in 0..height as usize {
        println!("row {j}/{height}");
        for i in 0..width as usize {

            if grid[j][i] != '.' { continue };

            let mut grid = grid.clone();
            let mut pos = pos;
            let mut dir_index = dir_index;

            grid[j][i] = '#';

            let mut locations_visited = Vec::new();

            'out:
            loop {
                let (x, y) = pos;
                grid[y as usize][x as usize] = 'X';

                if locations_visited.contains(&(pos, dir_index)) {
                    total += 1;
                    break 'out;
                } 

                locations_visited.push((pos, dir_index));
        
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
        }
    }



    println!("{}", total);
}
