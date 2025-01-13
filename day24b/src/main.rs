use std::io;
//use regex::Regex;
use std::collections::HashMap;
use rand::Rng;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    #[derive(Eq, Hash, PartialEq, Debug, Clone)]
    enum Op {
        Or,
        And,
        Xor,
    }

    let mut graph: Vec<(String, Op, String, String)> = Vec::new();
    let mut values: HashMap<String, bool> = HashMap::new();

    // non-idiomatic approach to reading all the input
    // process the top section of the input until we reach an empty line
    while let Some(Ok(line)) = lines.next() {
        //println!("{:?}", line);

        // empty line found, bail out
        if line.len() == 0 { break };

        let parts: Vec<_> = line.split(": ").collect();
        //println!("{} {}", pages[0], pages[1]);

        let node = parts[0].to_string();
        let value = parts[1].parse::<u32>().unwrap();

        values.insert(node, value == 1);
    }

    // process the bottom section of the input, and calculate the total as we go
    while let Some(Ok(line)) = lines.next() {
        //println!("{:?}", line);

        // empty line found, bail out
        if line.len() == 0 { break };

        let parts: Vec<_> = line.split(" ").collect();
        //println!("{} {}", pages[0], pages[1]);

        let in1 = parts[0].to_string();
        let in2 = parts[2].to_string();
        let out = parts[4].to_string();
        let op = match parts[1] {
            "OR" => Op::Or,
            "AND" => Op::And,
            "XOR" => Op::Xor,
            _ => panic!("Invalid Operator")
        };

        //println!("{} {:?} {} -> {}", in1, op, in2, out);

        graph.push((in1, op, in2, out));
        /*graph.entry((in1, op, in2))
            .or_insert(Vec::new())
            .push(out);*/
    }

    let mut swaps = Vec::new();

    let mut graph = graph.clone();
    let a = 7;
    let b = 89;
    let (a1, op1, b1, out1) = graph[a].clone();
    let (a2, op2, b2, out2) = graph[b].clone();
    graph[a] = (a1.clone(), op1.clone(), b1.clone(), out2.clone());
    graph[b] = (a2.clone(), op2.clone(), b2.clone(), out1.clone());
    println!("swapped {} <-> {}", out1, out2);
    swaps.push(out1);
    swaps.push(out2);

     let a = 18;
    let b = 59;
    let (a1, op1, b1, out1) = graph[a].clone();
    let (a2, op2, b2, out2) = graph[b].clone();
    graph[a] = (a1.clone(), op1.clone(), b1.clone(), out2.clone());
    graph[b] = (a2.clone(), op2.clone(), b2.clone(), out1.clone());
    println!("swapped {} <-> {}", out1, out2);
    swaps.push(out1);
    swaps.push(out2);

    let a = 128;
    let b = 152;
    let (a1, op1, b1, out1) = graph[a].clone();
    let (a2, op2, b2, out2) = graph[b].clone();
    graph[a] = (a1.clone(), op1.clone(), b1.clone(), out2.clone());
    graph[b] = (a2.clone(), op2.clone(), b2.clone(), out1.clone());
    println!("swapped {} <-> {}", out1, out2);
    swaps.push(out1);
    swaps.push(out2);

    let a = 24;
    let b = 47;
    let (a1, op1, b1, out1) = graph[a].clone();
    let (a2, op2, b2, out2) = graph[b].clone();
    graph[a] = (a1.clone(), op1.clone(), b1.clone(), out2.clone());
    graph[b] = (a2.clone(), op2.clone(), b2.clone(), out1.clone());
    println!("swapped {} <-> {}", out1, out2);
    swaps.push(out1);
    swaps.push(out2);

    swaps.sort();
    for swap in swaps {
        print!(",{}", swap);
    }
    println!();

    let mut rnd = rand::thread_rng();

    //let mut swap_counter = Vec::new();

    //for a in 0..graph.len() 
    {
        'back_to_b:
        //for b in a+1..graph.len() 
        {
            //print!("{} <-> {} ", a, b);
            let mut graph = graph.clone();

            /*let (a1, op1, b1, out1) = graph[a].clone();
            let (a2, op2, b2, out2) = graph[b].clone();
            graph[a] = (a1.clone(), op1.clone(), b1.clone(), out2.clone());
            graph[b] = (a2.clone(), op2.clone(), b2.clone(), out1.clone());*/

            let mut histogram = [0;46];

            for _i in 0..100 {
                let mut values: HashMap<String, bool> = HashMap::new();
        
                let random_x = rnd.gen::<u64>() & 0b111111111111111111111111111111111111111111111;
                let random_y = rnd.gen::<u64>() & 0b111111111111111111111111111111111111111111111;
        
                for i in 0..45 {
                    let x_name = format!("x{:02}", i);
                    let x_value = random_x & (1 << i) != 0;
                    values.insert(x_name, x_value);
        
                    let y_name = format!("y{:02}", i);
                    let y_value = random_y & (1 << i) != 0;
                    values.insert(y_name, y_value);
                }
                //println!("{:?}", values);
                //println!("{:?}", graph);
        
                let mut todo = graph.clone();
        
                let mut skip = false;
                while !todo.is_empty() {
                    let size_before = todo.len();
                    for gate in todo.clone() {
                        let (in1, op, in2, out) = &gate;
        
                        if values.contains_key(in1) && values.contains_key(in2) {                
                            let operand1 = values.get(in1).unwrap();
                            let operand2 = values.get(in2).unwrap();
        
                            let result = match *op {
                                Op::Or => *operand1 || *operand2,
                                Op::And => *operand1 && *operand2,
                                Op::Xor => *operand1 ^ *operand2
                            };
        
                            /*let out_list = graph.get(gate).unwrap();
        
                            //println!("PROCESSED {:?} -> {:?}", gate, out_list);
        
                            for out in out_list {
                                values.insert(out.to_string(), result);
                            }*/
                            values.insert(out.to_string(), result);
        
                            let index = todo.iter().position(|x| *x == gate).unwrap();
        
                            //println!("REMOVING {:?} {:?}", gate, todo.iter().nth(index).unwrap());
        
                            todo.remove(index);
        
                            //println!("{:?}\n", todo);
                        }
                    }

                    if todo.len() == size_before {
                        skip = true;
                        println!("skip");
                        //continue 'back_to_b;
                    }
                }
        
                if skip {
                    //break;
                }

                //println!("{:?}", values);
        
                let keys = values.keys().clone();
                let mut keys = keys.collect::<Vec<_>>();
                keys.sort();
                /*for key in keys {
                    let value = values.get(key).unwrap();
                    let value = if *value { 1 } else { 0 };
                    //println!("{}: {}", key, value);
                }*/
        
                let extract = | c: char | {
                    let xs = values.keys().filter(|s| s.chars().nth(0).unwrap() == c);
                    let mut xs = xs.collect::<Vec<_>>();
                    xs.sort();
                    xs.reverse();
        
                    let mut total = 0u64;
                    for x_key in xs {
                        let value = values.get(x_key).unwrap();
                        let value = if *value { 1 } else { 0 };
                        //print!("{}", value);
                        total = total * 2 + value;
                    }
        
                    return total;
                };
        
                let total_x = extract('x');
                let total_y = extract('y');
                let total_z = extract('z');
                let expected = total_x + total_y;
        
                /*println!("TOTAL: {total_z}");
                println!("{} + {} = {}", total_x, total_y, expected);
                println!("{:46b}", total_x);
                println!("{:46b}", total_y);
                println!("{:46b}", expected);
                println!("{:46b}", total_z);*/
        
                for i in 0..46 {
                    let z_value = total_z & (1 << i) != 0;
                    let expected_value = expected & (1 << i) != 0;
        
                    if z_value != expected_value {
                        histogram[i as usize] += 1;
                    }
                }
            }
        
            let total = histogram.iter().sum::<u32>();
            for i in 0..46 {
                println!("{:02}: {}", i, histogram[i as usize]);
            }
            println!("TOTAL: {}", total);
            //swap_counter.push((a, b, total));
            println!("{total}");
        }
    }

    //swap_counter.sort_by(|(_, _, c), (_, _, d)| c.cmp(d));

    /*println!();
    for row in swap_counter.iter().take(50) {
        println!("{:?}", row);
    }*/

}

/*trim the search space by looking at what things in the graph contribute to <= 13 bits
these are NOW all 100% correct all the time, so anything making those shouldn't be in a swap_counter
probably the same for all things contributing to bits over around 30/31
*/

/*
(128, 152, 754)
(24, 47, 1035)
(128, 198, 1082)
(214, 217, 1169)
(90, 133, 1179)
(37, 90, 1199)
(90, 123, 1241)
(90, 217, 1241)
(37, 214, 1260)
(47, 217, 1269)
(37, 47, 1317)
(37, 159, 1367)
(47, 90, 1367)
(47, 152, 1373)
(152, 214, 1374)
(47, 123, 1387)
(90, 198, 1387)
(47, 133, 1395)
(84, 217, 1396)
(90, 128, 1398)
(90, 152, 1398)
(78, 128, 1408)
(24, 128, 1421)
(199, 217, 1423)
(41, 217, 1444)
(38, 217, 1448)
(133, 217, 1448)
(128, 159, 1449)
(3, 217, 1466)
(123, 214, 1466)
(47, 177, 1467)
(128, 146, 1467)
(159, 217, 1470)
(15, 217, 1476)
(37, 195, 1481)
(175, 217, 1482)
(166, 217, 1487)
(37, 84, 1488)
(66, 123, 1492)
(133, 214, 1496)
(47, 60, 1503)
(47, 54, 1509)
(66, 217, 1512)
(37, 66, 1513)
(123, 217, 1513)
(48, 217, 1514)
(211, 217, 1517)
(37, 192, 1520)
(47, 214, 1522)
(66, 133, 1522)
*/
