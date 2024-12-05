use std::io;
//use regex::Regex;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let mut before: HashMap<i32, Vec<i32>> = HashMap::new();

    // non-idiomatic approach to reading all the input
    // process the top section of the input until we reach an empty line
    while let Some(Ok(line)) = lines.next() {
        //println!("{:?}", line);

        // empty line found, bail out
        if line.len() == 0 { break };

        let pages: Vec<i32> = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
        //println!("{} {}", pages[0], pages[1]);
        
        before
            .entry(pages[0]) // find the vector of pages after this page
            .or_insert(Vec::new())  // or make a new vector 
            .push(pages[1]);                              // add the new page
    }

    let mut total = 0;

    // process the bottom section of the input, and calculate the total as we go
    while let Some(Ok(line)) = lines.next() {
        let pages: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        
        let mut is_correctly_ordered = true;

        // loop through all pages in the list (except the last as it is correctly ordered if we get that far)
        for i in 0..pages.len()-1 {
            match before.get(&pages[i]) {
                // if current page has a entry in the before list, then check that all following pages in the list are present in it
                Some(list) => {
                    if !pages.iter().skip(i + 1).all(|item| list.contains(item)) {
                        is_correctly_ordered = false;
                        break;
                    }
                },
                // if current page isn't in the before list, can't have a correct ordering
                None => {
                    is_correctly_ordered = false;
                    break;
                }
            }
        }

        if is_correctly_ordered {
            total += pages[pages.len() / 2];
        }
    }

    println!("{}", total);
}
