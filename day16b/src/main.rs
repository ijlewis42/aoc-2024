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

    let mut costs: HashMap<(usize, usize, bool), (i128, Vec<(usize, usize, bool)>)>  = HashMap::new();
    costs.insert((start_pos.0, start_pos.1, true), (0, Vec::new()));

    let mut done: HashMap<(usize, usize, bool), (i128, Vec<(usize, usize, bool)>)>  = HashMap::new();

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

        let (node, _cost) = frontier.pop().unwrap();
        //let (sx, sy, sdir) = start;
        //let (ex, ey, edir) = end;

        for ((start, end), edge_cost) in &graph {
            //let (sx, sy, sdir) = start;
            //let (ex, ey, edir) = end;

            if start == node {
                let (cost_to_start, _from_list) = costs.get(&start).unwrap();
                let new_cost = edge_cost + cost_to_start;
                if costs.contains_key(&end) {
                    let (old_cost, from_list) = costs.get(&end).unwrap();

                    if *old_cost == new_cost {
                        let mut temp = from_list.clone();
                        temp.push(*start);
                        costs.insert(*end, (*old_cost, temp.clone()));
                        println!("MORE THAN ONE: FROM {:?} TO {:?}", temp.clone(), end);
                    } else if new_cost < *old_cost {
                        costs.insert(*end, (new_cost, vec!(*start)));
                    }
                } else {
                    costs.insert(*end, (new_cost, vec!(*start)));
                }
            }
        }        

        done.insert(*node, costs.get(node).unwrap().clone());

        //println!("{:?}", done);
    }

    //let ((start, end), cost) = costs.get(&end_pos).unwrap();

    let (cost1, _from_list1) = costs.get(&(end_pos.0, end_pos.1, false)).unwrap();
    let (cost2, _from_list2) = costs.get(&(end_pos.0, end_pos.1, true)).unwrap();

    let total = std::cmp::min(*cost1, *cost2);
    println!("{total}");

    let mut todo = Vec::new();
    if cost1 <= cost2 {
        todo.push((end_pos.0, end_pos.1, false));
    }
    if cost2 <= cost1 {
        todo.push((end_pos.0, end_pos.1, true));
    }

    let mut count = 0;
    while !todo.is_empty() {
        let pos = todo.pop().unwrap();
        let (x, y, _dir) = pos;
        
        if grid[y][x] != 'O' {
            count += 1;
            grid[y][x] = 'O';
        }

        let (_cost, from_list) = costs.get(&pos).unwrap();
        for node in from_list {
            todo.push(*node);
        }
    }

    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();

    println!("TOTAL: {count}");
}

// 134597 too high
// 134588