use std::io;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    let mut total = 0;

    let binding = lines.map(|line| line.unwrap()).collect::<Vec<_>>();
    let chunks = binding.chunks(4);

    // non-idiomatic approach to reading all the input
    for chunk in chunks {
        // deal with the nastiness of the line -- this would probably have been better to have been a regex that just matched the two numbers on each line
        fn grab_numbers(line : String, starting_chars_to_skip : usize, deliminator: &str) -> (i128, i128) {
            let button_line = line;
            let button_line = &button_line[starting_chars_to_skip..button_line.len()];
            let numbers = button_line.split(deliminator);
            let numbers = numbers.map(|x|x.parse::<i128>().unwrap());
            let numbers = numbers.collect::<Vec<_>>();

            return (numbers[0], numbers[1]);
        }

        // read the input from the chunk
        let (x1, y1) = grab_numbers(chunk[0].clone(), 12, ", Y+");
        let (x2, y2) = grab_numbers(chunk[1].clone(), 12, ", Y+");
        let (x_result, y_result) = grab_numbers(chunk[2].clone(), 9, ", Y=");
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
