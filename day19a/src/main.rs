use std::io;
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    // unwrap all the lines
    //let lines = lines.map(|line| line.unwrap() + " ");

    let mut towels = Vec::new();
    let mut first = true;
    let mut total = 0;
    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        if line.len() == 0 { continue; }

        //println!("{:?}", line.clone());
        // add each line as a vector of chars

        println!("{y}: {:?}", line.clone());

        if first {
            first = false;
            let temp = line.split(", ");
            let temp = temp.map(|s| s.chars().collect::<Vec<char>>());
            let temp = temp.collect::<Vec<_>>();
            towels = temp.clone();
            println!("{:?}", temp);
        } else {
            let line = line.chars().collect::<Vec<char>>();

            let mut overall_success = false;
            let mut tried = Vec::new();
            let mut todo = Vec::new();
            todo.push(0);

            while !todo.is_empty() {
                let current_index = todo.pop().unwrap();

                if tried.contains(&current_index) { continue; }

                tried.push(current_index);

                for towel in &towels {
                    let mut success = true;
                    for i in 0..towel.len() {
                        if current_index + i >= line.len() || line[current_index + i] != towel[i] {
                            success = false;
                            break;
                        }
                    }

                    if success {
                        let next_index = current_index + towel.len();
                        if next_index == line.len() {
                            overall_success = true;
                        } else {
                            if !todo.contains(&next_index) {
                                todo.push(current_index + towel.len());
                            }
                        }
                    }
                }
            }

            if overall_success {
                println!("SUCCESS!");
                total += 1;
            } else {
                println!("FAILURE");
            }
        }        
    }

    println!("TOTAL: {total}");
}