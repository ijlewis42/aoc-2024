use std::io;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    // vvv ALL AI GENERATED BECAUSE I COULDN'T GET RUST TO SHUT THE FUCK UP TONIGHT

    // Collect the lines into a Vec<Result<String, Error>>
    let lines_vec: Vec<Result<String, io::Error>> = lines.collect(); 

    // Handle potential errors during line reading
    let lines_vec: Vec<String> = lines_vec
        .into_iter()
        .map(|result| result.expect("Failed to read line")) 
        .collect();

    // Now you can safely chunk the lines
    let chunks = lines_vec.chunks(8);

    // ^^^ ALL AI GENERATED BECAUSE I COULDN'T GET RUST TO SHUT THE FUCK UP TONIGHT

    let mut keys = Vec::new();
    let mut locks = Vec::new();

    // Process the chunks
    for chunk in chunks {
        if chunk[0] == "#####" {
            println!("lock");

            let mut heights = Vec::new();
            for column in 0..5 {
                let mut height = -1;
                for line in chunk.iter().take(7) {
                    if line.chars().collect::<Vec<char>>()[column as usize] == '#' {
                        height += 1;
                    }
                }
                heights.push(height);
            }

            println!("{:?}", heights);
            locks.push(heights);
        } else {
            println!("key");

            let mut heights = Vec::new();
            for column in 0..5 {
                let mut height = 6;
                for line in chunk.iter().take(7) {
                    if line.chars().collect::<Vec<char>>()[column as usize] == '.' {
                        height -= 1;
                    }
                }
                heights.push(height);
            }

            println!("{:?}", heights);
            keys.push(heights);
        }
    }

    let mut total = 0;
    for lock in &locks {
        for key in &keys {
            let mut success = true; 
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    success = false;
                }
            }

            if success {
                total += 1;
            }
        }
    }

    println!("TOTAL: {total}");
}

