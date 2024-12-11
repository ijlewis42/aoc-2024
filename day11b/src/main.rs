use std::io;
//use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let line = lines.next().unwrap().unwrap();

    let values = line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();

    let mut copies = [HashMap::new(), HashMap::new()];

    for value in values.clone() {
        *copies[0]
            .entry(value)
            .or_insert(0u64)
            += 1;
    }

    println!("{:?}", copies[0]);

    let mut from_index = 0;
    let mut to_index = 1;

    for blink in 0..75 {
        println!("{blink}: ");

        //let start = Instant::now();

        copies[to_index] = HashMap::new();

        for (value, count) in copies[from_index].clone().iter() {
            let current = value;
    
            if *current == 0 {
                //println!("case 0");
                *copies[to_index].entry(1).or_insert(0) += count;
            } else if current.to_string().len() % 2 == 0 {
                //println!("case EVEN digits");
                let s = current.to_string();
                let left = s[0..s.len() / 2].parse::<u64>();
                let right = s[s.len() / 2..s.len()].parse::<u64>();
                //println!("{s} {left:?} {right:?}");
                *copies[to_index].entry(left.unwrap()).or_insert(0) += count;
                *copies[to_index].entry(right.unwrap()).or_insert(0) += count;
            } else {
                //println!("case OTHER");
                *copies[to_index].entry(current * 2024).or_insert(0) += count;
            }
        }

        from_index = (from_index + 1) % 2;
        to_index = (to_index + 1) % 2;
    }

    let total: u64 = copies[from_index].values().sum();
    println!("{}", total);
}
