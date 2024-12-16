use std::io;
use std::collections::HashMap;
//use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();


    let mut grid : Vec<Vec<(char, i128)>> = Vec::new();
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

        //grid.push(line);
        let line_with_costs = line.into_iter().map(|c| (c, 10000000000i128)).collect();
        grid.push(line_with_costs);
    }

    // size of grid
    //let height = grid.len() as i32;
    //let width = grid[0].len() as i32;

    let mut total = 0;

    let mut visited = Vec::new();

    let mut todo = Vec::new();

    let mut costs: HashMap<(usize, usize), i128>  = HashMap::new();
    costs.insert(start_pos, 0);

    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        // calculate new location
        let (nx, ny) = ((start_pos.0 as i32 + dx) as usize, (start_pos.1 as i32 + dy) as usize);

        let edge_cost = if dx == 1 && dy == 0 { 1 } else { 1000 };

        todo.push(((nx as usize, ny as usize), (dx, dy), edge_cost));
    }

    todo.sort_by(|((x, y), (dx, dy), edge_cost), ((x2, y2), (dx2, dy2), edge_cost2)| {
        let from_pos1 = ((*x as i32 - *dx) as usize, (*y as i32 - *dy) as usize);
        let cost1 = costs.get(&from_pos1).unwrap() + edge_cost;

        let from_pos2 = ((*x2 as i32 - *dx2) as usize, (*y2 as i32 - *dy2) as usize);
        let cost2 = costs.get(&from_pos2).unwrap() + edge_cost2;

        return cost1.cmp(&cost2);
    });

    while !todo.is_empty() {
        let (pos, dir, edge_cost) = todo.pop().unwrap();
        //let ((x, y), (odx, ody), cost_so_far) = todo.pop();
        let (x, y) = pos;
        let (odx, ody) = dir;

        //println!("{pos:?} {dir:?} {cost_so_far}");

        let (cell, _cost) = grid[y][x];
        if cell == '#' {
            //println!(" - hit wall");
            continue;
        } else if cell == 'E' {
            //println!(" - reached end");
            //total = cost_so_far;
            //println!("{total}");
            //continue;
        }

        if visited.contains(&(x, y)) {
            //println!(" - already been here");
            continue;
        }

        visited.push((x, y));
        
        let from_pos = ((x as i32 - odx) as usize, (y as i32 - ody) as usize);
        let cost_to_here = costs.get(&from_pos).unwrap() + edge_cost;
        if costs.contains_key(&pos) {
            let previous_value = costs.get(&pos).unwrap();
            costs.insert(pos, std::cmp::min(cost_to_here, *previous_value));
        } else {
            costs.insert(pos, cost_to_here);
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            // calculate new location
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);

            let edge_cost = if dx == odx && dy == ody { 1 } else { 1001 };

            todo.push(((nx as usize, ny as usize), (dx, dy), edge_cost));
        }

        todo.sort_by(|((x, y), (dx, dy), edge_cost), ((x2, y2), (dx2, dy2), edge_cost2)| {
            let from_pos1 = ((*x as i32 - *dx) as usize, (*y as i32 - *dy) as usize);
            let cost1 = costs.get(&from_pos1).unwrap() + edge_cost;
    
            let from_pos2 = ((*x2 as i32 - *dx2) as usize, (*y2 as i32 - *dy2) as usize);
            let cost2 = costs.get(&from_pos2).unwrap() + edge_cost2;
    
            return cost2.cmp(&cost1);
        });
        //println!("{todo:?}");
    }

    total = *costs.get(&end_pos).unwrap();

    println!("{total}");
}

// 134597 too high