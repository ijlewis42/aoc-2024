use std::io;
use std::collections::HashMap;
use std::cmp::max;

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

                // we could be going in any direction from the first antenna, so potential loop max is a little complicated
                let max = max(max(max(*x1, *y1), width - x1), height - y1);

                for k in -max..=max {
                //for k in -min(x1, y1)..=min(width - x1, height - y1) {
                    let (nx, ny) = (x1 - k * dx, y1 - k * dy);

                    // declaring the closure here, as if it was declared outside this loop, it would "still exist" (due to scope lifetimes) by the time grid needed to be mutated further down (see A and B)
                    let test = |x: i32, y: i32| {
                        x >= 0 && x < width && y >= 0 && y < height && grid[y as usize][x as usize] != '#'
                    };
    
                    // if a new spot for an antinode, add it and count it
                    if test(nx, ny) {                       // (A) test "ends" here, so immutable reference is "freed"
                        grid[ny as usize][nx as usize] = '#';    // (B) safe to use the mutable reference again
                        total += 1;
                    }    
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
