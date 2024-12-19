use std::io;
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    // unwrap all the lines
    let lines = lines.map(|line| line.unwrap() + " ");

    // concat all the strings
    let all = lines.collect::<String>();

    println!("{}", all.clone());


    // extract all the numbers via a regex
    let all_numbers = Regex::new(r"-?\d+").unwrap().captures_iter(&all)
                // convert them all to i128s
                .map(|c| c[0].parse::<i32>().unwrap())
                // collect them into a vector for easy retrieval
                .collect::<Vec<_>>();

    let mut grid : Vec<Vec<char>> = Vec::new();

    let width: i32 = 71;
    let height: i32 = 71;
    let data_to_read = 10240000;

    for y in 0..height {
        grid.push(Vec::new());
        for _x in 0..width {
            grid[y as usize].push('.');
        }
    }

    let start = (0i32, 0i32);
    let end = (width - 1, height - 1);

    for (i, pair) in all_numbers.chunks(2).enumerate() {
        if i == data_to_read { break; }

        let x = pair[0];
        let y = pair[1];

        println!("{i} coord: {x} {y}");

        grid[y as usize][x as usize] = '#';

        let mut visited = Vec::new();
        let mut todo = Vec::new();
        todo.push((start, 0));
    
        let mut success = false;

        while !todo.is_empty() {
            let (pos, cost) = todo.pop().unwrap();
            let (x, y) = pos;
    
            if grid[y as usize][x as usize] == '#' { continue; }
            if visited.contains(&pos) { continue; }
    
            visited.push(pos);
    
            if pos == end {
                success = true;
                break;
            }
    
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = x + dx;
                let ny = y + dy;
    
                if nx >= 0 && nx < width && ny >= 0 && ny < height {
                    todo.insert(0, ((nx, ny), cost + 1));
                }
            }
        }

        if !success {
            println!("FAILURE AT: {x} {y}");
            break;
        }
    }


    
    for y in 0..height {
        for x in 0..width {
            print!("{}", grid[y as usize][x as usize]);
        }
        println!();
    }
    println!();
}