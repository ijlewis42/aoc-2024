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
            // if an antenna at this location
            if c != '.' {
                // add it to the list of antennas of this type
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

    // loop through all the antenna types that were found
    for (_antenna_type, antennas) in antennas {
        // loop through each pair of antenna
        for (i, (x1, y1)) in antennas.iter().enumerate() {
            for (x2, y2) in antennas.iter().skip(i + 1) {
                // direction to second antenna from the first
                let (dx, dy) = (x2 - x1, y2 - y1);

                // locations of antinodes
                let (nx1, ny1) = (x1 - dx, y1 - dy);
                let (nx2, ny2) = (x2 + dx, y2 + dy);

                // I suspect the ugliness of this means it was really time to make grid a class
                // declaring the closure here, as if it was declared outside this loop, it would "still exist" (due to scope lifetimes) by the time grid needed to be mutated further down (see A and B)
                let test_and_update = |x: i32, y: i32, grid: &mut Vec<Vec<char>>| {
                    if x >= 0 && x < width && y >= 0 && y < height && grid[y as usize][x as usize] != '#' {
                        grid[y as usize][x as usize] = '#';
                        return true;
                    }
                    return false;
                };

                // if a new spot for an antinode, add it and count it                
                if test_and_update(nx1, ny1, &mut grid) {
                    total += 1;
                }
                // if a new spot for an antinode, add it and count it                
                if test_and_update(nx2, ny2, &mut grid) {
                    total += 1;
                }
            }
        }
    }

    /*// output the final grid
    for line in grid.clone() {
        println!("{}", line.iter().collect::<String>())
    }
    println!("");*/

    println!("{}", total);
}
