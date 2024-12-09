use std::io;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use aoc::*; // baby's first crate

fn concat_numbers(a: i64, b: i64) -> i64 {
    // this is pretty gross
    //return a.to_string() + b.to_string().parse::<i64>().unwrap();

    // so is this, but is _maybe_ faster
    let b_len = (b as f64).log10().floor() as i64 + 1; 
    return a * 10i64.pow(b_len as u32) + b;
}

#[derive(EnumIter, Clone, Copy)]
enum Operators {
    Add,
    Multiply,
    Concatenate,
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    let mut total = 0i64;

    // non-idiomatic approach to reading all the input
    for line in lines {
        let line = line.unwrap();

        let mut sections= line.split(": ");
        let wanted_total: i64 = sections.next().unwrap().parse::<i64>().unwrap();
        let numbers = sections.next().unwrap().split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

        // generate all permutations of our operators for the number of oeprators we need
        for indexes in permutations_from_iter(Operators::iter(), numbers.len() - 1) {
            let total_test = numbers.iter()
                // skip the first element (and initialise the fold with it), because we need to pair each subsequent element with an operator
                .skip(1)                
                // pair each element with an operator
                .zip(indexes)
                // calculate the total value
                .fold(numbers[0], |acc, (x, op) | {
                    use Operators::*;
                    match op {
                        Add =>          { acc + x },
                        Multiply =>     { acc * x },
                        Concatenate =>  { concat_numbers(acc, *x) }
                    }
                });

            /* less idiomatic, but still not terrible
            let mut total_test = numbers[0];
            for i in 1..numbers.len() {
                use Operators::*;
                match indexes[i - 1] {
                    Add =>          { total_test += numbers[i]; },
                    Multiply =>     { total_test *= numbers[i]; },
                    Concatenate =>  { total_test = concat_numbers(total_test, numbers[i]); }
                }
            }*/

            // test the total against what's expected, and add (and exit) if it matches
            if total_test == wanted_total {
                total += wanted_total;
                break;
            }
        }
    }

    println!("{}", total);
}
