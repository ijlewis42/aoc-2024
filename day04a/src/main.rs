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

    let word:Vec<char> = "XMAS".chars().collect();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut total = 0;

    // loop over the whole grid
    for y in 0..height {
        for x in 0..width {
            // look in all 8 directions
            for dx in [-1i32, 0, 1] {       // so gross looking
                for dy in [-1i32, 0, 1] {
                    // skip middle case (both offsets are zero)
                    if dx == 0 && dy == 0 { continue };

                    // assume success
                    let mut success = true;

                    // loop over all the characters in the word
                    for i in 0..word.len() as i32 {
                        // work out coordinates from (x, y) position, with an offset in current direction, with a distance equal to current character of word
                        let px: i32 = x + dx * i;
                        let py: i32 = y + dy * i;

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
                        total += 1;
                    }
                }
            }  
        }
    }

    println!("{}", total);
}
