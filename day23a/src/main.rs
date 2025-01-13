use std::io;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    // gunna build up the lists as I go
    let mut graph = HashMap::new();
    
    // non-idiomatic approach to reading all the input
    for line in lines {
        let line = line.unwrap();
        let pair = line.split("-");
        let pair = pair.map(|x| x.to_string()).collect::<Vec<String>>();

        graph.entry(pair[0].clone())
            .or_insert(Vec::new())
            .push(pair[1].clone());

        graph.entry(pair[1].clone())
            .or_insert(Vec::new())
            .push(pair[0].clone());
    }

    println!("{:?}", graph);

    let mut trios = Vec::new();

    for (computer1, connections1) in &graph {
        for computer2 in connections1 {
            let connections2 = graph.get(computer2).unwrap();

            for computer3 in connections2 {
                let connections3 = graph.get(computer3).unwrap();

                if connections3.contains(&computer1) {
                    let mut trio = [computer1, computer2, computer3];
                    trio.sort();
                    if !trios.contains(&trio) {
                        trios.push(trio);
                    }
                    //println!("{computer1} {computer2} {computer3}");
                }
            }
        } 
    }

    trios.sort();

    println!("{:?}", trios);
    for trio in &trios {
        println!("{:?}", trio);
    }

    println!("");

    let mut total = 0;
    for trio in trios.into_iter().filter(|[x, y, z]| 
            x.chars().nth(0).unwrap() == 't' || y.chars().nth(0).unwrap() == 't' || z.chars().nth(0).unwrap() == 't') {        
        println!("{:?}", trio);
        total += 1;
    }

    println!("TOTAL: {total}");

}