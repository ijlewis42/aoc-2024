use std::io;
use regex::Regex;
use std::collections::HashMap;

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
    let mut sequences = Vec::new();
    let mut diff_sequences = Vec::new();
    for number in all_numbers {
        //println!("\nNUMBER: {number}");

        let mut old_digit = number % 10;
        let mut new_number = number;
        let mut sequence = Vec::new();
        let mut diff_sequence = Vec::new();
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

            let new_digit = new_number % 10;
            //println!("{}: {} ({})", new_number, new_digit, new_digit - old_digit);
            sequence.push(new_number);
            diff_sequence.push(new_digit - old_digit);
            old_digit = new_digit;
        }
        //println!("{number}: {new_number}");
        //total += new_number;
        sequences.push(sequence);
        diff_sequences.push(diff_sequence);
    }
    //println!("{:?}", sequences);

    let mut quick_lookup = Vec::new();

    let mut all_chunks = Vec::new();
    for sequence in diff_sequences.clone() {
        let cccc = sequence.clone();
        let chunks = cccc.windows(4);
        let mut map: HashMap<[i64; 4], usize> = HashMap::new();

        for (index, chunk) in chunks.clone().enumerate() {
            all_chunks.push(chunk.to_vec());

            if !map.contains_key(chunk) {
                let temp:[i64;4] = [chunk[0], chunk[1], chunk[2], chunk[3]];

                map.insert(temp, index);
            }
        }
        quick_lookup.push(map);
    }

    println!("all_chunks LENGTH {}", all_chunks.len());

    let mut all_chunks = Vec::new();
    for a in -9..=9 {
        for b in -9..=9 {
            for c in -9..=9 {
                for d in -9..=9 {
                    all_chunks.push([a, b, c, d]);
                }
            }
        }
    }
    println!("all_chunks LENGTH {}", all_chunks.len());

    let mut best = 0;
    for (count, chunk) in all_chunks.iter().enumerate() {
        if count % 10000 == 0 {
            println!("{count}");
        }
        let mut subtotal = 0;
        for (index, sequence) in diff_sequences.clone().iter().enumerate() {
            let cccc = sequence.clone();
            let mut chunks = cccc.windows(4);
            //let chunks = chunks.collect::<Vec<_>>();
            //let found_index_maybe = chunks.position(|x| x == chunk);
            
            let found_index_maybe = quick_lookup[index].get(chunk);

            if let Some(found_index) = found_index_maybe {
                subtotal += sequences[index][found_index + 3] % 10;
                /*if chunk[0] == -2 {
                    println!("{:?} -> {} {} {}", chunk, index, found_index, sequences[index][found_index + 3] % 10);
                    //println!("{:?}", diff_sequences[index]);
                    /*for ii in found_index..found_index + 5 {
                        println!("{:?} {:?} {:?}", ii, diff_sequences[index][ii], sequences[index][ii]);
                    }*/
                }*/
            }
        }
        if subtotal > best {
            best = subtotal;
            println!("{:?} -> {}", chunk, subtotal);
        }
    }

    if [0, 1, 3, 4] == [5, 4, 2, 3] {
        println!("why?!!!");
    }

    total = best;

    println!("TOTAL: {total}");
}

