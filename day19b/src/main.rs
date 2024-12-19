use std::io;
//use regex::Regex;
use std::collections::HashMap;

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
            towels.sort_by(|a, b| (*a).len().cmp(&b.len()));
            println!("{:?}", towels);
        } else {
            let line = line.chars().collect::<Vec<char>>();

            let mut overall_success = false;
            let len = line.len();
            let mut totals: Vec<u128> = Vec::new();
            for _i in 0..len+1 {
                totals.push(0);
            }
            totals[0] = 1;

            for current_index in 0..line.len() {
                if current_index > 0 && totals[current_index] == 0 {
                    continue;
                }

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
                        totals[next_index] += totals[current_index];
                    }
                }
            }

            overall_success = totals[totals.len() - 1] > 0;
            if overall_success {
                println!("{:?}", totals);
                println!("SUCCESS! {}", totals[totals.len() - 1]);
                total += totals[totals.len() - 1];
            } else {
                println!("FAILURE");
            }
        }        
    }

    println!("TOTAL: {total}");
}