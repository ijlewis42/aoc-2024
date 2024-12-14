use std::io;
use std::collections::HashMap;

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

    let mut visited = Vec::new();

    for j in 0..height {
        for i in 0..width {
            if visited.contains(&(i, j)) {
                continue;
            }

            let plant = grid[j as usize][i as usize];

            println!("PLANT {plant}: {j} {i}");

            let mut region = Vec::new();


            let mut todo = Vec::new();
            todo.push((i, j));


            let mut area = 0;
            let mut perimeter = 0;
            while !todo.is_empty() {
                let (x, y) = todo.pop().unwrap();
   
                //println!("trying {y} {x}");

                // get the value at the current location
                let value = grid[y as usize][x as usize];

                if value != plant { continue; }

                // if location already visited, bail out, otherwise remember it
                if visited.contains(&(x, y)) { continue; }
                visited.push((x, y));

                //println!("-> visited {y} {x}");

                area += 1;

                let mut perimeter_to_add = 4;

                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    // calculate new location
                    let (nx, ny) = (x + dx, y + dy);

                    // check that new location is within grid bounds and that value there is correct 
                    if nx >= 0 && nx < width && ny >= 0 && ny < height && grid[ny as usize][nx as usize] == plant {
                        perimeter_to_add -= 1;
                    } 
                }
                perimeter += perimeter_to_add;

                region.push((x, y));


                // search in the four cardinal directions
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    // calculate new location
                    let (nx, ny) = (x + dx, y + dy);

                    // check that new location is within grid bounds and that value there is correct 
                    if nx >= 0 && nx < width && ny >= 0 && ny < height {
                        todo.push((nx, ny));
                    } 
                }
            }

            println!("GIVES AREA:  {area}");
            println!("GIVES PERIM: {perimeter}");
            total += area * perimeter;
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
