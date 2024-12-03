use std::io;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    // gunna build up the lists as I go
    let mut left : Vec<i32> = Vec::new();
    let mut right : Vec<i32> = Vec::new();

    // non-idiomatic approach to reading all the input
    for line in lines {
        let line = line.unwrap();
        let pair = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        //println!("{} {}", pair[0], pair[1]);

        // add each element of the pair to a different list
        left.push(pair[0]);
        right.push(pair[1]);
    }

    // sort the lists
    left.sort();
    right.sort();

    // honestly probably still prefer to use a for rather than a for_each here (and for is slightly less verbose and symbolly)
    /*let mut total = 0;
    for (x, y) in left.iter().zip(right.iter()) {
        let diff = (y - x).abs();
        //println!("{x} {y} -- {diff}");
        total += diff;
    }*/

    // idiomatic approach to finding difference between each pair in the list
    // totally didn't think to abs this the first time, so I failed day 1 part 1 on first try ;)
    /*let mut total = 0;
    left.iter().zip(right.iter()).for_each(|(x, y)| {
        total += (y - x).abs()
    });*/

    // more idiomatic
    let total : i32 = left.iter().zip(right.iter())
        .map(|(x, y)| (y - x).abs())
        .sum();

    println!("{}", total);
}
