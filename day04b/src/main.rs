use std::io;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    let mut grid : Vec<Vec<char>> = Vec::new();

    // non-idiomatic approach to reading all the input
    for line in lines {
        let line = line.unwrap();

        // add each line as a vector of chars
        grid.push(line.chars().collect());
    }

    // word as a vector of chars
    let word:Vec<char> = "MAS".chars().collect();

    // size of grid
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut total = 0;

    // loop over the whole grid
    for y in 0..height {
        for x in 0..width {
            let mut subtotal = 0;
            // look in four diagonal directions, but treat (x,y) as the CENTRE point, not the top-left point
            for dx in [-1i32, 1] {              // still so gross looking, and seems more silly with so few options now
                for dy in [-1i32, 1] {
                    // assume success
                    let mut success = true;

                    // loop over all the characters in the word
                    for i in 0..word.len() as i32 {
                        // work out coordinates from (x, y) position, with an offset in current direction, with a distance equal to current character of word
                        let px: i32 = x + dx * (i - 1);     // offset the index by -1 to treat (x,y) as CENTRE point
                        let py: i32 = y + dy * (i - 1);

                        // not success if out of bounds
                        if px < 0 || py < 0 || px >= width || py >= height {
                            success = false;
                            break;
                        }

                        // not success if character in grid doesn't match character in the word
                        if grid[py as usize][px as usize] != word[i as usize] {
                            success = false;
                            break;
                        }
                    } 

                    // count this found copy of the word
                    if success {
                        subtotal += 1;
                    }
                }
            }  

            // if found two copies with same centre, we have a X-"MAS"
            if subtotal == 2 {
                total += 1;
            }
        }
    }

    println!("{}", total);
}
