use std::io;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    let mut total = 0;

    let mut grid : Vec<String> = Vec::new();
    // non-idiomatic approach to reading all the input
    for line in lines {
        let line = line.unwrap();
        //let values = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        //println!("{:?}", values);

        // pair each adjacent value
        //let adjacents = values.windows(2);
        //println!("{:?}", adjacents.clone().collect::<Vec<_>>());

        //total += 0;

        grid.push(line);
    }

    let word = "XMAS";

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            for dx in [-1i32, 0, 1] {
                for dy in [-1i32, 0, 1] {
                    if dx == 0 && dy == 0 { continue };

                    let mut success = true;
                    for i in 0..word.len() {
                        let px: i32 = x as i32 + dx * (i as i32);
                        let py: i32 = y as i32 + dy * (i as i32);

                        if px < 0 || py < 0 || px >= (grid[0].len() as i32) || py >= (grid.len() as i32) {
                            success = false;
                            break;
                        }

                        if grid[py as usize].chars().nth(px as usize).unwrap() != word.chars().nth(i).unwrap() {
                            success = false;
                            break;
                        }
                    } 

                    if success {
                        total += 1;
                    }
                }
            }  
        }
    }

    println!("{}", total);
}
