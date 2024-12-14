use std::io;
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    // unwrap all the lines
    let lines = lines.map(|line| line.unwrap());

    // concat all the strings
    let all = lines.collect::<String>();

    // extract all the numbers via a regex
    let all_numbers = Regex::new(r"\d+").unwrap().captures_iter(&all)
                // convert them all to i128s
                .map(|c| c[0].parse::<i128>().unwrap())
                // collect them into a vector for easy retrieval
                .collect::<Vec<_>>();

    //println!("{all_numbers:?}");

    let mut total = 0;

    // group the numbers into groups of 6 elements
    let chunks = all_numbers.chunks(6);

    for chunk in chunks {
        // read the numbers from the chunk
        let [x1, y1, x2, y2, x_result, y_result] = chunk else { panic!("not the right amount of numbers in input") };
        let x_result = x_result + 10000000000000i128;
        let y_result = y_result + 10000000000000i128;

        //println!("{x1} {y1}");
        //println!("{x2} {y2}");
        //println!("{x_result} {y_result}\n");

        // code for solving simultaneous equations
        let b_numerator = x_result * y1 - y_result * x1;
        let b_denominator = y1 * x2 - x1 * y2;

        //println!("b = {b_numerator}/{b_denominator}");

        // only a valid solution we care about if this divides evenly
        if b_numerator % b_denominator == 0 {
            let b = b_numerator / b_denominator;
            let a_numerator = x_result * y1 - y1 * x2 * b; 
            let a_denominator = y1 * x1;

            //println!("a = {a_numerator}/{a_denominator}");

            // only a valid solution we care about if this divides evenly
            if a_numerator % a_denominator == 0 {
                let a = a_numerator / a_denominator;
                //println!("success {a} {b}");
                
                // add value of this solution to total
                total += a * 3 + b;
            }
        }
    }

    println!("TOTAL: {}", total);
}

// doing the maths: example

// 94a + 22b = 8400
// 34a + 67b = 5400

// 3196a + 748b = 285600
// 3196a + 6298b = 507600

// -5550b = -222000
// b = 40
// a = (285600 - 748 * 40)/3196 = 80

// generalising the maths

// Aa + Bb = C
// Da + Eb = F

// DAa + DBb = CD
// ADa + AEb = FA

// (DBb - AEb) = (CD - FA)
// b(DB - AE) = (CD - FA)
// b = (CD - FA) / (DB - AE)
// a = (CD - DBb) / DA

// I used different symbols because I'm an idiot

// A = x1, B = x2, C = x_result
// D = y1, E = y2, F = y_result
