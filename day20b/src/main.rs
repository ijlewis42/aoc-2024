use std::io;
use std::collections::HashMap;
//use std::collections::BinaryHeap;
use std::cmp::*;

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

    let mut saved = HashMap::new();

    for gy in 1..height-1 {
        println!("{gy}");
        for gx in 1..width - 1 {
            if grid_base[gy as usize][gx as usize] == '#' {
                for hy in gy..height-1 {
                    //print!(" {hy}");
                    for hx in gx..width - 1 {
                        if gx == hx && gy == hy { continue; }

                        let manhattan_dist = (gx - hx).abs() + (gy - hy).abs();

                        if manhattan_dist <= 1 {
                            if grid_base[hy as usize][hx as usize] != '#' {                                
                                println!("{gx} {gy} -> {hx} {hy}");

                                let mut grid = grid_base.clone();
                                for xx in min(gx, hx)..=max(gx, hx) {
                                    grid[gy as usize][xx as usize] = '.';
                                }
                                for yy in min(gy, hy)..=max(gy, hy) {
                                    grid[yy as usize][max(gx, hx) as usize] = '.';
                                }
                                //grid[gy as usize][gx as usize] = '.';
    
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
                                    saved.entry(base_cost - shortcut_cost)
                                        .and_modify(|e| *e += 1)
                                        .or_insert(1);      
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("BASE: {base_cost}");
    //println!("{:?}", saved);
    //saved.sort();
    //println!("{:?}", saved);
    let mut greater_or_equal_than_100 = saved.into_iter().filter(|(x, _c)| *x >= 0).collect::<Vec<_>>();
    greater_or_equal_than_100.sort_by(|(x, _c), (y, _d)| x.cmp(y));
    for (x, c) in &greater_or_equal_than_100 {
        println!("There are {c} cheats that save {x} picoseconds.");
    }
    //println!("{:?}", greater_or_equal_than_100);

    let sum = greater_or_equal_than_100.iter().map(|(_x, c)| c).sum::<i32>();

    println!("{:?}", sum);
}
