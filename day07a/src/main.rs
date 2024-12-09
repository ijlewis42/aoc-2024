use std::io;
use num::pow;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    let mut total = 0i64;

    // non-idiomatic approach to reading all the input
    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        let mut sections= line.split(": ");
        let wanted_total: i64 = sections.next().unwrap().parse::<i64>().unwrap();
        let numbers = sections.next().unwrap().split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

        let length: u32 = (numbers.len() - 1) as u32;

        let bit_length = i32::pow(2, length);
        for x in 0..bit_length {
            let mut total_test = numbers[0];

            for i in 1..numbers.len() {
                if 1 << (i - 1) & x == 0 {
                    total_test += numbers[i];
                } else {
                    total_test *= numbers[i];
                }
            }
            
            if total_test == wanted_total {
                total += wanted_total;
                println!("success! vvv");
                break;
            }
        }

        println!("{wanted_total} {:?}", numbers);
    }

    println!("{}", total);
}
