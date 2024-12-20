use std::io;
use std::collections::HashMap;
//use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();


    let mut grid : Vec<Vec<char>> = Vec::new();
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

        grid.push(line);
        //let line_with_costs = line.into_iter().map(|c| (c, 10000000000i128)).collect();
        //grid.push(line_with_costs);
    }

    // size of grid
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut graph = HashMap::new();

    //[derive(Eq)]
    //enum Direction { Horiz, Vert }

    for y in 0..height {
        for x in 0..width {
            if grid[y as usize][x as usize] != '#' {
                for (dx, dy, direction) in [(-1, 0, true), (1, 0, true), (0, -1, false), (0, 1, false)] {
                    // calculate new location
                    let (nx, ny) = ((x + dx) as usize, (y + dy) as usize);
    
                    if grid[ny][nx] != '#' {
                        graph.insert(((x as usize, y as usize, direction), (nx, ny, direction)), 1);
                        graph.insert(((x as usize, y as usize, true), (x as usize, y as usize, false)), 1000);
                        graph.insert(((x as usize, y as usize, false), (x as usize, y as usize, true)), 1000);
                    }
                }    
            }
        }
    }

    //println!("{graph:?}");

    let mut costs: HashMap<(usize, usize, bool), i128>  = HashMap::new();
    costs.insert((start_pos.0, start_pos.1, true), 0);

    let mut done: HashMap<(usize, usize, bool), i128>  = HashMap::new();

    // world's slowest djikstra

    //for _i in 0..10 {
    loop {
        let mut frontier = Vec::new();
        for ((start, end), cost) in &graph {
            //let (sx, sy, sdir) = start;
            //let (ex, ey, edir) = end;

            if costs.contains_key(&start) && !done.contains_key(&start) {
                frontier.push((start, costs.get(&start).unwrap()));
            }
        }
        frontier.sort_by(|(_, c1), (_, c2)| c2.cmp(&c1));

        //println!("{:?}", frontier);

        if frontier.is_empty() {
            break;
        }

        let (node, cost) = frontier.pop().unwrap();
        //let (sx, sy, sdir) = start;
        //let (ex, ey, edir) = end;

        for ((start, end), cost) in &graph {
            //let (sx, sy, sdir) = start;
            //let (ex, ey, edir) = end;

            if start == node {
                let new_cost = cost + costs.get(&start).unwrap();
                if costs.contains_key(&end) {
                    let old_cost = *costs.get(&end).unwrap();
                    costs.insert(*end, std::cmp::min(old_cost, new_cost));
                } else {
                    costs.insert(*end, new_cost);
                }
            }
        }        

        done.insert(*node, *costs.get(node).unwrap());

        //println!("{:?}", done);
    }

    //let ((start, end), cost) = costs.get(&end_pos).unwrap();

    let total = std::cmp::min(*costs.get(&(end_pos.0, end_pos.1, false)).unwrap(), *costs.get(&(end_pos.0, end_pos.1, true)).unwrap());

    println!("{total}");
}

// 134597 too high
// 134588