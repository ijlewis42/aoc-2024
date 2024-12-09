use std::io;
use num::pow;
use itertools::Itertools;
//use std::slice::range;

fn generate_combinations(numbers: &[i32], length: i32, current: Vec<i32>) {
    if current.len() == length as usize {
        println!("{:?}", current);
        return;
    }

    for &number in numbers {
        let mut new_vec = current.clone();
        new_vec.push(number);
        generate_combinations(numbers, length, new_vec);
    }
}

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

        /*let range = vec![0, 1, 2];
        let range = range.iter();
        let range = range.cartesian_product(0..bit_length);
        let range = range.collect::<Vec<_>>();

        println!("{range:?}");

        for combination in [0, 1, 2].iter().clone().(bit_length).multi_cartesian_product() {
            println!("{combination}");
        }*/
        let xxx = [0, 1, 2];
        //generate_combinations(&xxx, bit_length, Vec::new());

        //println!("BIT LENGTH {}", bit_length);
        for x in 0..bit_length*bit_length {
            //println!("BBB {:0width$b}", x, width = (bit_length * 2) as usize);

            let mut total_test = numbers[0];

            for i in 1..numbers.len() {
                let val = x >> ((i - 1) * 2) & 3; // 1 << (i - 1) & x;
                if val == 0 {
                    total_test += numbers[i];
                    //println!(" + {total_test}");
                } else if val == 1 {
                    total_test *= numbers[i];
                    //println!(" * {total_test}");
                } else if val == 2 {
                    total_test = (total_test.to_string() + &((&numbers[i]).to_string())).parse::<i64>().unwrap();
                    //println!(" # {total_test}");
                } else {
                    //println!(" BAIL");
                    total_test = 0;
                    break;
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
