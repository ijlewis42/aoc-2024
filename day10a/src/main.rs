use std::io;
//use std::collections::HashMap;
//use strum::IntoEnumIterator;
//use strum_macros::EnumIter;
//use aoc::*; // baby's first crate

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    let mut grid : Vec<Vec<i32>> = Vec::new();

    // non-idiomatic approach to reading all the input
    for line in lines {
        let line = line.unwrap();

        // add each line as a vector of chars
        grid.push(line.chars().map(|c| 
            c.to_digit(10)
            .and_then(|x| Some (x as i32))
            .unwrap_or(-1i32)).collect());
    }

    // word as a vector of chars
    let word:Vec<char> = "XMAS".chars().collect();

    // size of grid
    let height = grid.len();
    let width = grid[0].len();

    let mut total = 0;


    let mut all_todo = Vec::new();
    
    for j in 0..height {
        for i in 0..width {
            if grid[j][i] == 0 {
                all_todo.push((i, j));
            }
        }
    }

    while all_todo.len() > 0 {
        let mut todo = Vec::new();
        let mut visited = Vec::new();

        todo.push(all_todo.pop().unwrap());

        while todo.len() > 0 {
            if let Some((x, y)) = todo.pop() {
                let value = grid[y][x];
    
                if visited.contains(&(x, y)) { continue; }
                visited.push((x, y));
    
                if value == 9 {
                    total += 1;
                } else {
                    if x > 0 && grid[y][x - 1] == value + 1 {
                        todo.push((x - 1, y));
                    }
                    if x < width - 1 && grid[y][x + 1] == value + 1 {
                        todo.push((x + 1, y));
                    }
                    if y > 0 && grid[y - 1][x] == value + 1 {
                        todo.push((x, y - 1));
                    }
                    if y < height - 1 && grid[y + 1][x] == value + 1 {
                        todo.push((x, y + 1));
                    }
                }
            }
        }
    }

    // output the final grid
    for line in grid.clone() {
        println!("{}", line.iter().map(|d| d.to_string()).collect::<String>())
    }
    println!("");

    println!("{total}");

}
