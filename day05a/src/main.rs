use std::io;
//use regex::Regex;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    let mut total = 0;

    let mut firstSection = true;

    let mut before: HashMap<i32, Vec<i32>> = HashMap::new();

    // non-idiomatic approach to reading all the input
    for line in lines {
        let line = line.unwrap();
        //println!("{:?}", line);

        if line.len() == 0 
        {
            firstSection = false;
            continue;
        }

        if firstSection {
            let parts: Vec<i32> = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect();

            println!("{} {}", parts[0], parts[1]);
            before.entry(parts[0]).or_insert(Vec::new()).push(parts[1]);
            /*if before.contains_key(&parts[0]) {
                before.get(&parts[0]).unwrap().push(parts[1]);
            } else {
                before.insert(parts[0], Vec::new());
            }*/
        } else {
            let parts: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

            let mut success = true;
            for i in 0..parts.len()-1 {
                for j in i+1..parts.len() {
                    match before.get(&parts[i]) {
                        Some(list) => {
                            if !list.contains(&parts[j]) {
                                success = false;
                                println!("NOT {} before {}", parts[i], parts[j]);
                            } else {
                                println!("{} before {}", parts[i], parts[j]);
                            }
                        },
                        None => {
                            success = false;
                            println!("NO MATCH");
                        }
                    } 
                }
            }

            if success {
                total += parts[parts.len() / 2];
                println!("woo!");
            } else {
                println!("boo!");
            }
        }


    }

    println!("{}", total);
}
