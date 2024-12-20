use std::io;
//use std::collections::HashMap;
//use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();


    let mut grid_base : Vec<Vec<char>> = Vec::new();
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    
    // non-idiomatic approach to reading all the input
    for (y, line) in lines.enumerate() {
        let line = line.unwrap();
        let line = line.chars().collect::<Vec<char>>();

        // add each line as a vector of chars
        for (x, c) in line.clone().into_iter().enumerate() {
            if c == 'S' {
                start_pos = (x, y);
            } else if c == 'E' {
                end_pos = (x, y);
            }
        }

        grid_base.push(line);
        //let line_with_costs = line.into_iter().map(|c| (c, 10000000000i128)).collect();
        //grid.push(line_with_costs);
    }

    // size of grid
    let height = grid_base.len() as i32;
    let width = grid_base[0].len() as i32;
    
    //println!("{graph:?}");
    let mut base_cost = 0;

    //let mut visited = Vec::new();

    let mut todo = Vec::new();
    todo.push((start_pos.0 as i32, start_pos.1 as i32, 0));

    let mut grid = grid_base.clone();

    while !todo.is_empty() {
        let (x, y, cost) = todo.pop().unwrap();

        if grid[y as usize][x as usize] == '#' {
            continue;
        }

        if (x as usize, y as usize) == end_pos {
            base_cost = cost;
            break;
        }

        grid[y as usize][x as usize] = '#';

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;

            todo.insert(0, (nx, ny, cost + 1));
        }
    }

    let mut saved = Vec::new();

    for gy in 1..height-1 {
        println!("{gy}");
        for gx in 1..width - 1 {
            let mut grid = grid_base.clone();
            if grid[gy as usize][gx as usize] == '#' {
                grid[gy as usize][gx as usize] = '.';

                /*for y in 0..height {
                    for x in 0..width {
                        print!("{}", grid[y as usize][x as usize]);
                    }
                    println!();
                }
                println!();*/
                
                
                let mut shortcut_cost = 0;

                let mut todo = Vec::new();
                todo.push((start_pos.0 as i32, start_pos.1 as i32, 0));
            
                while !todo.is_empty() {
                    let (x, y, cost) = todo.pop().unwrap();
            
                    if grid[y as usize][x as usize] == '#' {
                        continue;
                    }
            
                    if (x as usize, y as usize) == end_pos {
                        shortcut_cost = cost;
                        break;
                    }
            
                    grid[y as usize][x as usize] = '#';
            
                    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        let nx = x + dx;
                        let ny = y + dy;
            
                        todo.insert(0, (nx, ny, cost + 1));
                    }
                }

                if base_cost - shortcut_cost > 0 {
                    //println!("SAVED: {}", base_cost - shortcut_cost);      
                    saved.push(base_cost - shortcut_cost);      
                }
            }
        }
    }

    println!("BASE: {base_cost}");
    //println!("{:?}", saved);
    //saved.sort();
    //println!("{:?}", saved);
    let greater_or_equal_than_100 = saved.into_iter().filter(|x| *x >= 100).collect::<Vec<_>>();
    println!("{:?}", greater_or_equal_than_100.len());
}
