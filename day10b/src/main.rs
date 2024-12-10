use std::io;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    let mut grid : Vec<Vec<Option<i32>>> = Vec::new();

    // non-idiomatic approach to reading all the input
    for line in lines {
        let line = line.unwrap();

        // add each line as a vector of chars
        grid.push(line.chars().map(|c| 
                c.to_digit(10)
                .and_then(|x| Some (x as i32))
            )
            .collect());
    }

    // size of grid
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    // find all locations in grid that contain a 0 (i.e. are a trailhead)
    let mut trailheads = Vec::new();  
    for j in 0..height {
        for i in 0..width {
            if grid[j as usize][i as usize] == Some(0) {
                trailheads.push((i, j));
            }
        }
    }

    let mut total = 0;

    for trailhead in trailheads {
        let mut todo = Vec::new();
        // for Part B, just don't need to keep track of locations already visited
        //let mut visited = Vec::new(); 

        // search only from one trailhead at a time, so the locations visited don't interfere with each other
        todo.push(trailhead);

        // keep looping until no more locations to visit
        while !todo.is_empty() {
            let (x, y) = todo.pop().unwrap();

            // if location already visited, bail out, otherwise remember it
            //if visited.contains(&(x, y)) { continue; }
            //visited.push((x, y));

            // get the value at the current location
            let value = grid[y as usize][x as usize].unwrap();

            // if the value is 9, it can't be any higher, so count it as a summit
            if value == 9 {
                total += 1;
            } else {
                // search in the four cardinal directions
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    // calculate new location
                    let (nx, ny) = (x + dx, y + dy);

                    // check that new location is within grid bounds and that value there is correct 
                    if nx >= 0 && nx < width && ny >= 0 && ny < height && grid[ny as usize][nx as usize] == Some(value + 1) {
                        todo.push((nx, ny));
                    } 
                }
            }
        }
    }

    // output the final grid
    /*for line in grid.clone() {
        println!("{}", line.iter().map(|d| match d {
            Some(d) => d.to_string(),
            None => ".".to_string()
        }).collect::<String>());
    }
    println!("");*/

    println!("{total}");

}
