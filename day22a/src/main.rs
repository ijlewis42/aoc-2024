use std::io;
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    // unwrap all the lines
    let lines = lines.map(|line| line.unwrap() + " ");

    // concat all the strings
    let all = lines.collect::<String>();

    //println!("{}", all.clone());


    // extract all the numbers via a regex
    let all_numbers = Regex::new(r"-?\d+").unwrap().captures_iter(&all)
                // convert them all to i128s
                .map(|c| c[0].parse::<i64>().unwrap())
                // collect them into a vector for easy retrieval
                .collect::<Vec<_>>();

    let mut total = 0;
    for number in all_numbers {
        //println!("\nNUMBER: {number}");

        let mut new_number = number;
        for _i in 0..2000 {
            let temp = new_number * 64;
            new_number = new_number ^ temp;
            new_number = new_number % 16777216;

            let temp = new_number / 32;
            new_number = new_number ^ temp;
            new_number = new_number % 16777216;

            let temp = new_number * 2048;
            new_number = new_number ^ temp;
            new_number = new_number % 16777216;

        }
        println!("{number}: {new_number}");
        total += new_number;
    }
    println!("TOTAL: {total}");
}
