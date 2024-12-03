use std::io;
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    let mut total = 0;
    let mut multiply = true;

    // non-idiomatic approach to reading all the input
    for line in lines {
        let line = line.unwrap();
        //println!("{:?}", line);

        let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't|do").unwrap();

        for cap in re.captures_iter(&line) {
            // the whole match
            let whole = &cap[0];
            //println!("{whole}");

            match whole {
                "do" => {
                    multiply = true;
                    println!("-> multiply on");
                },
                "don't" => {
                    multiply = false;
                    println!("-> multiply off");
                },
                _ => {
                    // throw away the "mul(" and the ")", leaving us with just xxx,yyy
                    let slice = &whole[4..whole.len() - 1];
                    //println!("{slice}");

                    // extract the numbers
                    let parts: Vec<i32> = slice.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

                    // add this one to the total if multiply is true
                    if multiply
                    {
                        total += parts[0] * parts[1];
                    }
                }
            }
        }
    }

    println!("{}", total);
}
