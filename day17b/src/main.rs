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
                .map(|c| c[0].parse::<i64>().unwrap())
                // collect them into a vector for easy retrieval
                .collect::<Vec<_>>();

    let mut a = all_numbers[0];
    let mut b = all_numbers[1];
    let mut c = all_numbers[2];
    let program = all_numbers[3..].to_vec();

    //let mut out = "";

    let result_needed:Vec<i64> = program.clone(); //.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    // foolishly searched from 0..45719000000

    // string 17 bounds: 281474972088833 < 17len < 281476372088832
    // first string of length 16 is made when a=35184372088832
                                        
    //for i in (140734372088832..175934372088832).step_by(1000000000) { 
    for i in (164278763988832..164278774588832).step_by(1) { 
            
        a = i;

        /*if i % 1000000 == 0 {
            println!("{i}");
        }*/

        let mut result = Vec::new();
        let mut success = true;
        
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

            //println!("{operand} {combo}");

            match instruction {
                0 => {
                    //println!("AA {a} {b} {c} PC: {pc}");                    
                    a = a / 2f32.powf(combo as f32) as i64;
                    //println!("BB {a} {b} {c} PC: {pc}");
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
                        //println!("{a} {b} {c} PC: {pc}");
                        continue;
                    }
                },
                4 => {
                    b = b ^ c;
                },
                5 => {
                    //print!("{},", combo % 8);                
                    result.push(combo % 8);
                    //println!("pushing: {}", combo % 8);

                    if result[result.len() - 1] != result_needed[result.len() - 1] {
                        success = false;
                        //println!("NEXT!");
                        //break;
                    }
                },
                6 => {
                    b = a / 2f32.powf(combo as f32) as i64;
                },
                7 => {
                    c = a / 2f32.powf(combo as f32) as i64;
                },
                _ => {
                    panic!("invalid opcode");
                }

            }

            pc += 2;

            //println!("{a} {b} {c} PC: {pc}");
        }

        if success && result.len() == result_needed.len() {
            println!("SUCCESS {i} -> {result:?}");
            break;
        } else {
            //if i % 1000 == 0 {
                //println!("FAILURE {i} -> {result:?} of length {}/{}", result.len(), result_needed.len());
            //}
        }
//break;
    }



}
