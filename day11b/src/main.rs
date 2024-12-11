use std::io;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let line = lines.next().unwrap().unwrap();

    // read the raw starting values
    let values = line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();

    // can use a histogram, because order isn't important
    // need two, so we can process the old one into the new
    let mut copies = [HashMap::new(), HashMap::new()];

    // add a histogram of all initial values to the first hashmap
    for value in values.clone() {
        *copies[0]
            .entry(value)
            .or_insert(0u64)
            += 1;
    }
    //println!("{:?}", copies[0]);

    // indexes of the two different histograms
    let mut from_index = 0;
    let mut to_index = 1;

    for _blink in 0..75 {
        // empty the histogram we are going to insert into
        copies[to_index].clear();

        for (value, count) in copies[from_index].clone().iter() {
            // calculate new values, these have to be vectors, not arrays as the array length is part of the type
            let new_values = if *value == 0 {
                // 0 -> 1
                vec![1]
            } else if value.to_string().len() % 2 == 0 {
                // even lengthed numbers become two halves, e.g. 1234 -> 12 34
                let s = value.to_string();
                let left = s[0..s.len() / 2].parse::<u64>().unwrap();
                let right = s[s.len() / 2..s.len()].parse::<u64>().unwrap();
                vec![left, right]
            } else {
                // x -> x * 2024
                vec![value * 2024]
            };

            // add all new values
            for new_value in new_values {
                *copies[to_index].entry(new_value).or_insert(0) += count;
            }
        }

        from_index = (from_index + 1) % 2;
        to_index = (to_index + 1) % 2;
    }

    // sum all histogram counts
    let total: u64 = copies[from_index].values().sum();

    println!("{}", total);
}
