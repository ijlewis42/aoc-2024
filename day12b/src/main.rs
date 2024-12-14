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

    let mut regions = Vec::new();

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
            //let mut perimeter = 0;
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

            let mut x = i;
            let mut y = j;
            let mut line_count = 0;
            let mut index = 0;
            let mut outsides = HashMap::new();

            loop { //for _i in 0..100 {
                if index == 0 {
                    index = 1;
                    if x + 1 >= width { outsides.entry(None).or_insert(true); }
                    let outside = if y > 0 { Some(grid[(y - 1) as usize][x as usize]) } else { None };
                    outsides.entry(outside).or_insert(true);
                    while x + 1 < width && grid[y as usize][(x + 1) as usize] == plant {
                        let outside = if y > 0 { Some(grid[(y - 1) as usize][x as usize]) } else { None };
                        outsides.entry(outside).or_insert(true);
                        x += 1;
                        if y > 0 && grid[(y - 1) as usize][x as usize] == plant { index = 3; break; }
                    }
                    if x + 1 >= width { outsides.entry(None).or_insert(true); }
                    let outside = if y > 0 { Some(grid[(y - 1) as usize][x as usize]) } else { None };
                    outsides.entry(outside).or_insert(true);
                    line_count += 1;
                    //println!("MOVED RIGHT TO {} {}", y + 1, x + 1);
                }

                if index == 1 {
                    index = 2;
                    if y + 1 >= height { outsides.entry(None).or_insert(true); }
                    let outside = if x + 1 < width { Some(grid[y as usize][(x + 1) as usize]) } else { None };
                    outsides.entry(outside).or_insert(true);
                    while y + 1 < height && grid[(y + 1) as usize][x as usize] == plant {
                        let outside = if x + 1 < width { Some(grid[y as usize][(x + 1) as usize]) } else { None };
                        outsides.entry(outside).or_insert(true);
                        y += 1;
                        if x + 1 < width && grid[y as usize][(x + 1) as usize] == plant { index = 0; break; }
                    }
                    if y + 1 >= height { outsides.entry(None).or_insert(true); }
                    let outside = if x + 1 < width { Some(grid[y as usize][(x + 1) as usize]) } else { None };
                    outsides.entry(outside).or_insert(true);
                    line_count += 1;
                    //println!("MOVED DOWN  TO {} {}", y + 1, x + 1);
                }

                if index == 2 {
                    index = 3;
                    if x == 0 { outsides.entry(None).or_insert(true); }
                    let outside = if y + 1 < height { Some(grid[(y + 1) as usize][x as usize]) } else { None };
                    outsides.entry(outside).or_insert(true);
                    while x > 0 && grid[y as usize][(x - 1) as usize] == plant {                    
                        let outside = if y + 1 < height { Some(grid[(y + 1) as usize][x as usize]) } else { None };
                        outsides.entry(outside).or_insert(true);
                        x -= 1;
                        if y + 1 < height && grid[(y + 1) as usize][x as usize] == plant { index = 1; break; }
                    }
                    if x == 0 { outsides.entry(None).or_insert(true); }
                    let outside = if y + 1 < height { Some(grid[(y + 1) as usize][x as usize]) } else { None };
                    outsides.entry(outside).or_insert(true);
                    line_count += 1;
                    //println!("MOVED LEFT  TO {} {}", y + 1, x + 1);
                }

                if index == 3 {
                    index = 0;
                    if y == 0 { outsides.entry(None).or_insert(true); }
                    let outside = if x > 0 { Some(grid[y as usize][(x - 1) as usize]) } else { None };
                    outsides.entry(outside).or_insert(true);
                    while y > 0 && grid[(y - 1) as usize][x as usize] == plant {
                        let outside = if x > 0 { Some(grid[y as usize][(x - 1) as usize]) } else { None };
                        outsides.entry(outside).or_insert(true);
                        y -= 1;
                        if x > 0 && grid[y as usize][(x - 1) as usize] == plant { index = 2; break; }
                    }
                    if y == 0 { outsides.entry(None).or_insert(true); }
                    let outside = if x > 0 { Some(grid[y as usize][(x - 1) as usize]) } else { None };
                    outsides.entry(outside).or_insert(true);
                    line_count += 1;
                    //println!("MOVED UP    TO {} {}", y + 1, x + 1);
    
                }

                if x == i && y == j  && line_count % 2 == 0 { break; }

                //break;
            }

            regions.push((plant, area, line_count, region));

            //println!("GIVES AREA:  {area}");
            //println!("GIVES LINES: {line_count}");
            //println!("OUTSIDES: {:?}", outsides.keys());
            total += area * line_count;

            if !outsides.contains_key(&None) && outsides.keys().len() == 1 {
                //println!("NEED TO ADD THIS ONE MORE");
                for a in 0..regions.len() {
                    let (reg_plant, reg_area, reg_perim, reg_region) = &regions[a];
                    if *reg_plant == outsides.keys().collect::<Vec<_>>()[0].unwrap() {
                        if reg_region.contains(&(i, j - 1)) {
                            println!("ADDING TO REGION {reg_plant} with AREA {reg_area}");
                            //regions[a] = (*reg_plant, *reg_area, *reg_perim + line_count, reg_region);
                            regions[a].2 = *reg_perim + line_count;
                            total += area * line_count;
                        }
                    }
                }
            }
            
            //break;
        }
        //break;
    }

    let mut new_total = 0;
    for a in 0..regions.len() {
        let (reg_plant, reg_area, reg_perim, _reg_region) = &regions[a];
    
        println!("{reg_plant}: {reg_area} * {reg_perim} = {}", reg_area * reg_perim);
        new_total += reg_area * reg_perim;
    }
    
    // output the final grid
    /*for line in grid.clone() {
        println!("{}", line.iter().map(|d| match d {
            Some(d) => d.to_string(),
            None => ".".to_string()
        }).collect::<String>());
    }
    println!("");*/

    println!("{total} {new_total}");
}

// 41004 too low
// 826904 too low
// 833336 too low
