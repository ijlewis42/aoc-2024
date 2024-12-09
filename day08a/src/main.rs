use std::io;
use std::collections::HashMap;
//use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    let mut grid : Vec<Vec<char>> = Vec::new();

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    
    // non-idiomatic approach to reading all the input
    for (y, line) in lines.enumerate() {
        let line = line.unwrap();
        
        let line = line.chars().collect::<Vec<char>>();

        // add each line as a vector of chars
        grid.push(line.clone());

        for (x, c) in line.into_iter().enumerate() {
            if c != '.' {
                antennas
                .entry(c)
                .or_insert(Vec::new())
                .push((x as i32, y as i32));
            }
        }
    }

    // size of grid
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut total = 0;

    for (antenna_type, antennas) in antennas {
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let (x1, y1) = antennas[i];
                let (x2, y2) = antennas[j];

                let (dx, dy) = (x2 - x1, y2 - y1);

                let (nx1, ny1) = (x1 - dx, y1 - dy);
                let (nx2, ny2) = (x2 + dx, y2 + dy);

                if nx1 >= 0 && nx1 < width && ny1 >= 0 && ny1 < height {
                    if grid[ny1 as usize][nx1 as usize] != '#' {
                        grid[ny1 as usize][nx1 as usize] = '#';
                        total += 1;
                    }
                }

                if nx2 >= 0 && nx2 < width && ny2 >= 0 && ny2 < height {
                    if grid[ny2 as usize][nx2 as usize] != '#' {
                        grid[ny2 as usize][nx2 as usize] = '#';
                        total += 1;
                    }
                }
            }

        }
    }

    /*for line in grid.clone() {
        println!("{}", line.iter().collect::<String>())
    }
    println!("");*/


    println!("{}", total);
}
