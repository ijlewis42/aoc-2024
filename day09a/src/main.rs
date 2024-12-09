use std::io;
use std::collections::HashMap;
use std::cmp::max;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let line = lines.next().unwrap().unwrap();
    let line = line.chars().collect::<Vec<char>>();

    let mut disc = Vec::new();
    
    for (x, c) in line.into_iter().enumerate() {
        if x % 2 == 0 {
            for _i in 0..c.to_digit(10).unwrap() {
                disc.push(Some(x / 2));
            }
        } else {
            for _i in 0..c.to_digit(10).unwrap() {
                disc.push(None);
            }
        }
    }

    loop {
        let hole_to_fill = disc.iter().position(|&item| item == None);

        let hole_to_fill = match hole_to_fill {
            None => break,
            Some(index) => index
        };

        println!("{hole_to_fill}");

        disc[hole_to_fill] = disc.pop().unwrap();

        while disc[disc.len() - 1] == None {
            disc.pop();
        }
    }


    println!("{:?}", disc);

    let total = disc.iter().enumerate().map(|(i, x)| i * x.unwrap());
    let total = total.sum::<usize>();

    println!("{}", total);
}
