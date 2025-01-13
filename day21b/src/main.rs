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

            if ey == sy && ex == sx {
                ret.push(((sx, sy), ""));
                return ret;
            }

            if ey < sy && keypad[sy - 1][sx] != '#' {
                ret.push(((sx, sy - 1), "^"));
            } else if ey > sy && keypad[sy + 1][sx] != '#' {
                ret.push(((sx, sy + 1), "v"));
            }

            if ex < sx && keypad[sy][sx - 1] != '#' {
                ret.push(((sx - 1, sy), "<"));
            } else if ex > sx && keypad[sy][sx + 1] != '#' {
                ret.push(((sx + 1, sy), ">"));
            }

            return ret;
        };

        let search2 = | keypad: Vec<Vec<char>>, (sx, sy): (usize, usize), (ex, ey): (usize, usize), sequence_so_far: String | {
            if keypad[sy][sx] == '<' && keypad[ey][ex] == '>' {
                return ">>";
            }
            if keypad[sy][sx] == '>' && keypad[ey][ex] == '<' {
                return "<<";
            }

            if keypad[sy][sx] == 'v' && keypad[ey][ex] == 'A' {
                return ">^";
            }
            if keypad[sy][sx] == 'A' && keypad[ey][ex] == 'v' {
                return "v<";
            }

            if keypad[sy][sx] == '^' && keypad[ey][ex] == '<' {
                return "v<";
            }
            if keypad[sy][sx] == '<' && keypad[ey][ex] == '^' {
                return ">^";
            }

            if keypad[sy][sx] == '^' && keypad[ey][ex] == '>' {
                return ">v";
            }
            if keypad[sy][sx] == '>' && keypad[ey][ex] == '^' {
                return "^<";
            }


            if keypad[sy][sx] == '<' && keypad[ey][ex] == 'A' {
                return ">>^";
            }

            if keypad[sy][sx] == 'A' && keypad[ey][ex] == '<' {
                return "v<<";
            }

            if ey == sy && ex == sx {
                return "";
            }

            if ey < sy && keypad[sy - 1][sx] != '#' {
                return "^";
            } else if ey > sy && keypad[sy + 1][sx] != '#' {
                return "v";
            }

            if ex < sx && keypad[sy][sx - 1] != '#' {
                return "<";
            } else if ex > sx && keypad[sy][sx + 1] != '#' {
                return ">";
            }

            return "";
        };

        let mut sequences_step1: Vec<((usize, usize), String, HashMap<String, u64>)> = Vec::new();
        sequences_step1.push((start_pos, "".to_string(), HashMap::new()));

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
                let ((x, y), sequence_so_far, histo_so_far) = todo.pop().unwrap();

                let mut histo_so_far = histo_so_far.clone();
                let more = search(keypad.clone(), (x, y), (ex, ey), sequence_so_far.clone());
                for (pos, new_additive_sequence) in more {
                    if pos == end_pos {
                        let a_index = sequence_so_far.rfind('A');
                        let index = match a_index {
                            Some(index) => index + 1,
                            None => 0
                        };
                        let other_bit = &sequence_so_far[index..];
                        *(histo_so_far.entry(other_bit.to_string() + new_additive_sequence + "A")
                            .or_insert(0u64))
                            += 1;
                        sequences_step1.push((pos, sequence_so_far.clone() + new_additive_sequence + "A", histo_so_far.clone()));
                    } else {
                        todo.push((pos, sequence_so_far.clone() + new_additive_sequence, histo_so_far.clone()));
                    }
                }
            }            
        }

        for sequence in &sequences_step1 {
            println!("A {:?}", sequence);
        }
        //let lookup = HashMap::new();

        let mut minimum_length = 1000000000000000000u64;

        for keypad_count in 0..26 {
            //print!("{keypad_count} ");
            io::stdout().flush();
            //println!("\nBEFORE: {}", sequences_step1.len());
            let lengths = sequences_step1.iter().map(|(_pos, sequence, _histo)| sequence.len());
            let min_length = lengths.min().unwrap();
            sequences_step1.retain(|(_pos, x, _histo)| x.len() == min_length);
            //println!("AFTER: {} -- {}", sequences_step1.len(), sequences_step1[0].1.len());

            let keypad_old = vec!("#####", "##^A#", "#<v>#", "#####");
            let keypad = keypad_old.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
            let start_pos = (3, 1);
    
            let mut sequences_step2 = Vec::new();  
    
            minimum_length = 1000000000000000000u64;
            for (iii, previous_sequence_data) in sequences_step1.iter().enumerate() {
                //println!("{iii}");
                let (_start_pos, previous_sequence, previous_histo) = previous_sequence_data;
                //let mut sequences = Vec::new();
                //sequences.push((start_pos, "".to_string(), HashMap::new()));
        
                let line = previous_sequence;
                let mut start_pos = start_pos;
                let mut sequence_so_far = "".to_string();
                let mut histo = HashMap::new();

                for (subsequence, count) in previous_histo {
                    for c in subsequence.chars() {
                        //println!("searching for {c}");
                        let end_pos = keypad_old.iter().enumerate().find_map(|(y, row)| row.find(c).and_then(|x| Some((x, y)))).unwrap();
                        let (ex, ey) = end_pos;
                        //println!("{ex} {ey}");
                        //let (sx, sy) = start_pos;
        
                        let (x, y) = start_pos;
            
                        let more = search2(keypad.clone(), (x, y), (ex, ey), sequence_so_far.clone());
                        //sequence_so_far = sequence_so_far.clone() + &(more.to_string() + "A").repeat(*count as usize);

                        *(histo.entry(more.to_string() + "A")
                            .or_insert(0u64))
                            += count;
                        
                        start_pos = end_pos;
                    }
                }
                //println!("Sequence: {:?} ({})", sequence_so_far, sequence_so_far.len());
                let mut subtotal = 0;
                for (subsequence, count) in &histo {
                    //println!("  {}: {}", subsequence, count);
                    subtotal += subsequence.len() as u64 * count;
                }       
                println!("  LENGTH: {}", subtotal);

                minimum_length = std::cmp::min(minimum_length, subtotal);

                sequences_step2.push((start_pos, sequence_so_far, histo));
            }    
            sequences_step1 = sequences_step2.clone();

            for sequence in &sequences_step2 {
                println!("B {:?}", sequence);
            }
        }

        let mult = numbers_only.collect::<String>().parse::<u64>().unwrap();
        let lengths = sequences_step1.iter().map(|(_pos, _sequence, histo)| histo.values().sum::<u64>());
        let length_min = minimum_length; // lengths.min().unwrap() as u64;
        let subtotal = mult * length_min;

        println!("SUBTOTAL: {mult} * {length_min} = {subtotal}");
        total += subtotal;
    }

    println!("TOTAL: {total}");
}

// 902078982798550 too high
// 351261151537816 too high



// 3 -> 3
// 32 -> 16
// 768 -> 768
// 116391936 -> 26738688


// 129A
// 0
// BEFORE: 6
// AFTER: 6 -- 14
// 0
// 1
// 2
// 3
// 4
// 5
// 1
// BEFORE: 6
// AFTER: 2 -- 30
// 0
// 1
// 2
// BEFORE: 2
// AFTER: 2 -- 74
// 0
// 1
// 3
// BEFORE: 2
// AFTER: 2 -- 190
// 0
// 1
// 4
// BEFORE: 2
// AFTER: 1 -- 478
// 0
// 5
// BEFORE: 1
// AFTER: 1 -- 1232
// 0
// 6
// BEFORE: 1
// AFTER: 1 -- 3158
// 0
// 7
// BEFORE: 1
// AFTER: 1 -- 8114
// 0
// 8
// BEFORE: 1
// AFTER: 1 -- 20832
// 0
// 9
// BEFORE: 1
// AFTER: 1 -- 53504
// 0
// 10
// BEFORE: 1
// AFTER: 1 -- 137398
// 0
// 11
// BEFORE: 1
// AFTER: 1 -- 352860
// 0
// 12
// BEFORE: 1
// AFTER: 1 -- 906178
// 0
// 13
// BEFORE: 1
// AFTER: 1 -- 2327176
// 0
// 14
// BEFORE: 1
// AFTER: 1 -- 5976448
