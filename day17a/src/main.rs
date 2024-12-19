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
    let all_numbers = Regex::new(r"-?\d+").unwrap().captures_iter(&all)
                // convert them all to i128s
                .map(|c| c[0].parse::<i32>().unwrap())
                // collect them into a vector for easy retrieval
                .collect::<Vec<_>>();

    let mut a = all_numbers[0];
    let mut b = all_numbers[1];
    let mut c = all_numbers[2];
    let program = all_numbers[3..].to_vec();

    //let mut out = "";

    let mut pc = 0;
    while pc + 1 < program.len() {
        let instruction = program[pc];
        let operand = program[pc + 1];

        let combo = match operand {
            0..=3 => operand,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("reserved operand")
        };

        match instruction {
            0 => {
                a = a / 2f32.powf(combo as f32) as i32;
            },
            1 => {
                b = b ^ operand;
            },
            2 => {
                b = combo % 8;
            }
            3 => {
                if a == 0 {
                    // nop
                } else {
                    pc = operand as usize;
                    continue;
                }
            },
            4 => {
                b = b ^ c;
            },
            5 => {
                print!("{},", combo % 8);                
            },
            6 => {
                b = a / 2f32.powf(combo as f32) as i32;
            },
            7 => {
                c = a / 2f32.powf(combo as f32) as i32;
            },
            _ => {
                panic!("invalid opcode");
            }

        }

        pc += 2;
    }


}
