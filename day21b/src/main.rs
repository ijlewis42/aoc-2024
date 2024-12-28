//use core::num;
use std::io;
use std::collections::*;
use std::io::Write;
fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    // unwrap and collect all the lines
    let lines = lines.map(|line| line.unwrap());
    //let lines = lines.collect::<Vec<_>>();

    let mut total = 0;

    // loop through all the remaining lines of the file (each containing a single design)
    for line in lines {
        // initialise our dynamic programming style array to have 1 way to make a zero length design, and 0 ways to make all the other lengths
        let mut subtotal= 0;

        println!("{line}");

        let numbers_only = line.chars().filter(|c|c.is_ascii_digit());

        let keypad_old = vec!("#####", "#789#", "#456#", "#123#", "##0A#", "#####");
        let keypad = keypad_old.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
        let start_pos = (3, 4);

        let search = | keypad: Vec<Vec<char>>, (sx, sy): (usize, usize), (ex, ey): (usize, usize), sequence_so_far: String | {
            let mut ret = Vec::new();

            /*if keypad[sy][sx] == '#' {
                return ret;
            }*/

            if keypad[sy][sx] == '<' && keypad[ey][ex] == '>' {
                ret.push(((ex, ey), sequence_so_far.clone() + ">>"));
                return ret;
            }
            if keypad[sy][sx] == '>' && keypad[ey][ex] == '<' {
                ret.push(((ex, ey), sequence_so_far.clone() + "<<"));
                return ret;
            }

            if keypad[sy][sx] == 'v' && keypad[ey][ex] == 'A' {
                ret.push(((ex, ey), sequence_so_far.clone() + ">^"));
                return ret;
            }
            if keypad[sy][sx] == 'A' && keypad[ey][ex] == 'v' {
                ret.push(((ex, ey), sequence_so_far.clone() + "v<"));
                return ret;
            }

            if keypad[sy][sx] == '^' && keypad[ey][ex] == '<' {
                ret.push(((ex, ey), sequence_so_far.clone() + "v<"));
                return ret;
            }
            if keypad[sy][sx] == '<' && keypad[ey][ex] == '^' {
                ret.push(((ex, ey), sequence_so_far.clone() + ">^"));
                return ret;
            }

            if keypad[sy][sx] == '^' && keypad[ey][ex] == '>' {
                ret.push(((ex, ey), sequence_so_far.clone() + ">v"));
                return ret;
            }
            if keypad[sy][sx] == '>' && keypad[ey][ex] == '^' {
                ret.push(((ex, ey), sequence_so_far.clone() + "^<"));
                return ret;
            }


            if keypad[sy][sx] == '<' && keypad[ey][ex] == 'A' {
                ret.push(((ex, ey), sequence_so_far.clone() + ">>^"));
                //ret.push(((ex, ey), sequence_so_far.clone() + ">^>"));
                return ret;
            }

            if keypad[sy][sx] == 'A' && keypad[ey][ex] == '<' {
                ret.push(((ex, ey), sequence_so_far.clone() + "v<<"));
                //ret.push(((ex, ey), sequence_so_far.clone() + "<v<"));
                return ret;
            }

            if ey == sy && ex == sx {
                ret.push(((sx, sy), sequence_so_far.clone()));
            }

            if ey < sy && keypad[sy - 1][sx] != '#' {
                ret.push(((sx, sy - 1), sequence_so_far.clone() + "^"));
            } else if ey > sy && keypad[sy + 1][sx] != '#' {
                ret.push(((sx, sy + 1), sequence_so_far.clone() + "v"));
            }

            if ex < sx && keypad[sy][sx - 1] != '#' {
                ret.push(((sx - 1, sy), sequence_so_far.clone() + "<"));
            } else if ex > sx && keypad[sy][sx + 1] != '#' {
                ret.push(((sx + 1, sy), sequence_so_far.clone() + ">"));
            }

            return ret;
        };

        let search2 = | keypad: Vec<Vec<char>>, (sx, sy): (usize, usize), (ex, ey): (usize, usize), sequence_so_far: String | {
            let mut ret = Vec::new();

            /*if keypad[sy][sx] == '#' {
                return ret;
            }*/

            if keypad[sy][sx] == '<' && keypad[ey][ex] == '>' {
                ret.push(">>");
                return ret;
            }
            if keypad[sy][sx] == '>' && keypad[ey][ex] == '<' {
                ret.push("<<");
                return ret;
            }

            if keypad[sy][sx] == 'v' && keypad[ey][ex] == 'A' {
                ret.push(">^");
                return ret;
            }
            if keypad[sy][sx] == 'A' && keypad[ey][ex] == 'v' {
                ret.push("v<");
                return ret;
            }

            if keypad[sy][sx] == '^' && keypad[ey][ex] == '<' {
                ret.push("v<");
                return ret;
            }
            if keypad[sy][sx] == '<' && keypad[ey][ex] == '^' {
                ret.push(">^");
                return ret;
            }

            if keypad[sy][sx] == '^' && keypad[ey][ex] == '>' {
                ret.push(">v");
                return ret;
            }
            if keypad[sy][sx] == '>' && keypad[ey][ex] == '^' {
                ret.push("^<");
                return ret;
            }


            if keypad[sy][sx] == '<' && keypad[ey][ex] == 'A' {
                ret.push(">>^");
                //ret.push(((ex, ey), sequence_so_far.clone() + ">^>"));
                return ret;
            }

            if keypad[sy][sx] == 'A' && keypad[ey][ex] == '<' {
                ret.push("v<<");
                //ret.push(((ex, ey), sequence_so_far.clone() + "<v<"));
                return ret;
            }

            if ey == sy && ex == sx {
                ret.push("");
            }

            if ey < sy && keypad[sy - 1][sx] != '#' {
                ret.push("^");
            } else if ey > sy && keypad[sy + 1][sx] != '#' {
                ret.push("v");
            }

            if ex < sx && keypad[sy][sx - 1] != '#' {
                ret.push("<");
            } else if ex > sx && keypad[sy][sx + 1] != '#' {
                ret.push(">");
            }

            return ret;
        };

        let mut sequences_step1 = Vec::new();
        sequences_step1.push((start_pos, "".to_string()));

        for c in line.chars() {
            //println!("searching for {c}");
            let end_pos = keypad_old.iter().enumerate().find_map(|(y, row)| row.find(c).and_then(|x| Some((x, y)))).unwrap();
            let (ex, ey) = end_pos;
            //println!("{dx} {dy}");
            //let (sx, sy) = start_pos;

            let mut todo = Vec::new();
            for sss in sequences_step1.clone() {
                todo.push(sss);
            }
            sequences_step1.clear();

            while !todo.is_empty() {
                let ((x, y), sequence_so_far) = todo.pop().unwrap();

                let more = search(keypad.clone(), (x, y), (ex, ey), sequence_so_far);
                for (pos, sequence) in more {
                    if pos == end_pos {
                        sequences_step1.push((pos, sequence.clone() + "A"));
                    } else {
                        todo.push((pos, sequence));
                    }
                }
            }            
        }

        // for sequence in &sequences_step1 {
        //     println!("A {:?}", sequence);
        // }
        //let lookup = HashMap::new();

        for keypad_count in 0..2 {
            print!("{keypad_count} ");
            io::stdout().flush();
            println!("\nBEFORE: {}", sequences_step1.len());
            let lengths = sequences_step1.iter().map(|(_pos, sequence)| sequence.len());
            let min_length = lengths.min().unwrap();
            sequences_step1.retain(|(_pos, x)| x.len() == min_length);
            println!("AFTER: {} -- {}", sequences_step1.len(), sequences_step1[0].1.len());

            let keypad_old = vec!("#####", "##^A#", "#<v>#", "#####");
            let keypad = keypad_old.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
            let start_pos = (3, 1);
    
            let mut sequences_step2 = Vec::new();  
    
            for (iii, previous_sequence_data) in sequences_step1.iter().enumerate() {
                println!("{iii}");
                let (_start_pos, previous_sequence) = previous_sequence_data;
                let mut sequences = Vec::new();
                sequences.push((start_pos, "".to_string()));
        
                let line = previous_sequence;
    
                for c in line.chars() {
                    //println!("searching for {c}");
                    let end_pos = keypad_old.iter().enumerate().find_map(|(y, row)| row.find(c).and_then(|x| Some((x, y)))).unwrap();
                    let (ex, ey) = end_pos;
                    //println!("{ex} {ey}");
                    //let (sx, sy) = start_pos;
        
                    let mut todo = Vec::new();
                    for sss in sequences.clone() {
                        todo.push(sss);
                    }
                    sequences.clear();
        
                    while !todo.is_empty() {
                        let ((x, y), sequence_so_far) = todo.pop().unwrap();
        
                        let more = search(keypad.clone(), (x, y), (ex, ey), sequence_so_far);
                        for (pos, sequence) in more {
                            if pos == end_pos {
                                sequences.push((pos, sequence.clone() + "A"));
                                //println!("{:?} {}", pos, sequence.clone() + "A");
                            } else {
                                //todo.push((pos, sequence));
                                //println!("Didn't push: {:?} {:?} {}", pos, end_pos, sequence.clone());
                            }
                        }
                    }            
                }
                for sequence in sequences {
                    sequences_step2.push(sequence);
                }
            }    
            sequences_step1 = sequences_step2;
        }


        // for sequence in &sequences_step2 {
        //     println!("B {:?}", sequence);
        // }

        let mult = numbers_only.collect::<String>().parse::<u64>().unwrap();
        let lengths = sequences_step1.iter().map(|(_pos, sequence)| sequence.len());
        let subtotal = mult * (lengths.min().unwrap() as u64);

        println!("SUBTOTAL: {subtotal}");
        total += subtotal;
    }

    println!("TOTAL: {total}");
}

// 029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
//       <vA<AA>>^AvAA<^A>Av<<A>>^AvA^A<vA>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A
// 980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
//       v<<A>>^AAAvA^A<vA<AA>>^AvAA<^A>Av<<A>A>^AAAvA<^A>A<vA>^A<A>A
// 179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
//       v<<A>>^A<vA<A>>^AAvAA<^A>Av<<A>>^AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A
// 456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
//       v<<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>Av<<A>A>^AAvA<^A>A
// 379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
//       v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A



// 143536 too high
// 136780 hooray


// 3 -> 3
// 32 -> 16
// 768 -> 768
// 116391936 -> 26738688