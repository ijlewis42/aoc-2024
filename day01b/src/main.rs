use std::io;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    // gunna build up the lists / maps as I go
    let mut left : Vec<i32> = Vec::new();
    let mut right = HashMap::new();

    // non-idiomatic approach to reading all the input
    // my failure to understand the errors I was getting in part 1 paid off here I feel ;)
    for line in lines {
        let line = line.unwrap();
        let pair = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        //println!("{} {}", pair[0], pair[1]);

        // add each left element of the pair to a list
        left.push(pair[0]);

        // add each right element of the pair to a hashmap histogram for each reference later
        *right.entry(pair[1]).or_insert(0) += 1;
    }

    /*let mut total = 0;

    // look up all values from the left list, in the right hashmap
    // and sum the total
    for x in left.iter() {
        let similarity_score = x * right.get(x).unwrap_or(&0);
        //println!("{x} {y} -- {similarity_score}");
        total += similarity_score;
    }*/

    // look up all values from the left list, in the right hashmap, and sum the total (idiomatically)
    let total : i32 = left.iter()
        .map(|x| x * right.get(x).unwrap_or(&0))
        .sum();

    println!("{}", total);
}
