use std::io;
//use std::collections::HashMap;
//use std::cmp::max;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let line = lines.next().unwrap().unwrap();
    let line = line.chars().collect::<Vec<char>>();

    let mut disc = Vec::new();
    
    for (x, c) in line.into_iter().enumerate() {
        let how_many = c.to_digit(10).unwrap();
        if x % 2 == 0 {
            disc.push((Some(x / 2), how_many));
        } else {
            disc.push((None, how_many));
        }
    }

    println!("{:?}", disc);

    /*for (x, count) in disc.clone() {
        match x {
            None => for _i in 0..count { print!("."); },
            Some(c) => for _i in 0..count { print!("{c}"); }
        }
    }
    println!();*/

    let mut index = 0;

    loop {
        //println!("AT INDEX {index}");

        let hole_to_fill = disc.iter().skip(index).position(|&(item, _how_many)| item == None);

        let hole_to_fill = match hole_to_fill {
            None => break,
            Some(index_to_fill) => index_to_fill + index
        };

        let size_of_gap = disc[hole_to_fill].1;

        //println!("hoel to fill {hole_to_fill}");

        let from_index = disc.iter().skip(index).rev().position(|&(item, how_many)| item != None && how_many <= size_of_gap);

        //println!("from_index {from_index:?}");

        match from_index {
            None => {
            },
            Some (from_index) => {
                let from_index = disc.len() - 1 - from_index;
                let clone1 = disc[from_index].clone();
                let (_, count) = clone1;

                let mut old = disc[from_index].clone();
                old.0 = None;
                disc[from_index] = old; 
                //println!("{from_index}");
                //println!("{old:?}");

                if count < size_of_gap {
                    disc[hole_to_fill] = clone1;
                    disc.insert(hole_to_fill + 1, (None, size_of_gap - count));
                    index = hole_to_fill;
                } else {
                    disc[hole_to_fill] = clone1;
                    index = hole_to_fill;
                }
            }
        }

        while let (None, _how_many) = disc[disc.len() - 1] {
            disc.pop();
        }

        /*println!("{:?}", disc);
        for (x, count) in disc.clone() {
            match x {
                None => for _i in 0..count { print!("."); },
                Some(c) => for _i in 0..count { print!("{c}"); }
            }
        }
        println!();*/

        index += 1;

        //break;
    }


    println!("{:?}", disc);

    for (x, count) in disc.clone() {
        match x {
            None => for i in 0..count { print!("."); },
            Some(c) => for i in 0..count { print!("{c}"); }
        }
    }
    println!();

    //let total = 0;
    let mut vec = Vec::new();
    for (x, count) in disc.clone() {
        match x {
            None => for _i in 0..count { vec.push(0); },
            Some(c) => for _i in 0..count { vec.push(c); }
        }
    }
    println!();

    let total = vec.iter().enumerate().map(|(i, x)| i * x);
    let total = total.sum::<usize>();

    println!("{}", total);
}

// first guess: 6239783431260
// second: 6239783302560