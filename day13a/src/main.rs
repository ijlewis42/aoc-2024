use std::io;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    let mut total = 0;

    let binding = lines.map(|line| line.unwrap()).collect::<Vec<_>>();
    let chunks = binding.chunks(4);

    // non-idiomatic approach to reading all the input
    for chunk in chunks {
        // doing this the ugly way because I don't want to do the (simple) maths

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
        let x_result = numbers[0];
        let y_result = numbers[1];

        println!("{x1} {y1}");
        println!("{x2} {y2}");
        println!("{x_result} {y_result}\n");

        let mut min = 10000000000000i128;
        let mut best = (0, 0);
        for press_count in 1..=100 {
            for a_press_count in 0..=press_count {
                for b_press_count in 0..=press_count {
                    let x_total = x1 * a_press_count + x2 * b_press_count;
                    let y_total = y1 * a_press_count + y2 * b_press_count;
                    //let total = x_total + y_total;
    
                    if x_total == x_result && y_total == y_result {
                        let cost = a_press_count * 3 + b_press_count;
    
                        if cost < min {
                            min = cost;
                            best = (a_press_count, b_press_count);
                        }
                    }
                }
            }    
        }

        println!("BEST: {:?}", best );

        let (best_a, best_b) = best;
        if best_a != 0 && best_b != 0 {
            total += min;
        }
    }

    println!("TOTAL: {}", total);
}
