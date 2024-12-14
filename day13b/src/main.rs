use std::io;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    let mut total = 0;

    let binding = lines.map(|line| line.unwrap()).collect::<Vec<_>>();
    let chunks = binding.chunks(4);

    // non-idiomatic approach to reading all the input
    for chunk in chunks {
        let button_line = chunk[0].clone();
        let button_line = &button_line[12..button_line.len()];
        let numbers = button_line.split(", Y+");
        let numbers = numbers.map(|x|x.parse::<i128>().unwrap());
        let numbers = numbers.collect::<Vec<_>>();
        let x1 = numbers[0];
        let y1 = numbers[1];

        let button_line = chunk[1].clone();
        let button_line = &button_line[12..button_line.len()];
        let numbers = button_line.split(", Y+");
        let numbers = numbers.map(|x|x.parse::<i128>().unwrap());
        let numbers = numbers.collect::<Vec<_>>();
        let x2 = numbers[0];
        let y2 = numbers[1];

        let button_line = chunk[2].clone();
        let button_line = &button_line[9..button_line.len()];
        let numbers = button_line.split(", Y=");
        let numbers = numbers.map(|x|x.parse::<i128>().unwrap());
        let numbers = numbers.collect::<Vec<_>>();
        let x_result = numbers[0] + 10000000000000i128;
        let y_result = numbers[1] + 10000000000000i128;

        println!("{x1} {y1}");
        println!("{x2} {y2}");
        println!("{x_result} {y_result}\n");

        let b_numerator = x_result * y1 - y_result * x1;
        let b_denominator = y1 * x2 - x1 * y2;

        println!("b = {b_numerator}/{b_denominator}");

        if b_numerator % b_denominator == 0 {
            let b = b_numerator / b_denominator;
            let a_numerator = x_result * y1 - y1 * x2 * b; 
            let a_denominator = y1 * x1;

            println!("a = {a_numerator}/{a_denominator}");

            if a_numerator % a_denominator == 0 {
                let a = a_numerator / a_denominator;
                println!("success {a} {b}");
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
