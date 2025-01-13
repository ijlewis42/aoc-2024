use std::io;
//use regex::Regex;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    #[derive(Eq, Hash, PartialEq, Debug)]
    enum Op {
        Or,
        And,
        Xor,
    }

    let mut graph: HashMap<(String, Op, String), Vec<String>> = HashMap::new();
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

        graph.entry((in1, op, in2))
            .or_insert(Vec::new())
            .push(out);
    }

    println!("{:?}", graph);

    let mut todo = graph.keys().clone().collect::<Vec<_>>();

    while !todo.is_empty() {
        for gate in todo.clone() {
            let (in1, op, in2) = gate;

            if values.contains_key(in1) && values.contains_key(in2) {                
                let operand1 = values.get(in1).unwrap();
                let operand2 = values.get(in2).unwrap();

                let result = match *op {
                    Op::Or => *operand1 || *operand2,
                    Op::And => *operand1 && *operand2,
                    Op::Xor => *operand1 ^ *operand2
                };

                let out_list = graph.get(gate).unwrap();

                println!("PROCESSED {:?} -> {:?}", gate, out_list);

                for out in out_list {
                    values.insert(out.to_string(), result);
                }

                let index = todo.iter().position(|x| *x == gate).unwrap();

                println!("REMOVING {:?} {:?}", gate, todo.iter().nth(index).unwrap());

                todo.remove(index);

                println!("{:?}\n", todo);
            }
        }
    }

    println!("{:?}", values);

    let keys = values.keys().clone();
    let mut keys = keys.collect::<Vec<_>>();
    keys.sort();
    for key in keys {
        let value = values.get(key).unwrap();
        let value = if *value { 1 } else { 0 };
        println!("{}: {}", key, value);
    }

    let zs = values.keys().filter(|s| s.chars().nth(0).unwrap() == 'z');
    let mut zs = zs.collect::<Vec<_>>();
    zs.sort();
    zs.reverse();

    let mut total = 0u128;

    for z_key in zs {
        let value = values.get(z_key).unwrap();
        let value = if *value { 1 } else { 0 };
        print!("{}", value);
        total = total * 2 + value;
    }
    println!("");

    println!("TOTAL: {total}");
}
