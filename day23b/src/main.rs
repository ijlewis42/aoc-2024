use std::io;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    // gunna build up the lists as I go
    let mut graph = HashMap::new();
    let mut nodes = Vec::new();
    
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

        if !nodes.contains(&pair[0]) {
            nodes.push(pair[0].clone());
        }
        if !nodes.contains(&pair[1]) {
            nodes.push(pair[1].clone());
        }
    }

    println!("{:?}", graph);

    let mut connections = nodes.clone().iter().map(|n| vec!(n.clone())).collect::<Vec<Vec<String>>>();

    while !connections.is_empty() {
        let mut bigger_connections = Vec::new();

        for group in &connections {
            for node in &nodes {
                if group.contains(node) { continue; }

                let mut success = true;
                for other in group {
                    if !graph.get(node).unwrap().contains(&other) {
                        success = false;
                    }
                }

                if success {
                    let mut new_group = group.clone();
                    new_group.push(node.clone());
                    new_group.sort();
                    if !bigger_connections.contains(&new_group) {
                        bigger_connections.push(new_group);
                    }
                }
            }
        }

        connections = bigger_connections;

        println!("{:?} x{:?}", connections.len(), connections[0].len());
        let mut temp = connections[0].clone();
        temp.sort();
        println!("{:?}\n", temp);
        for computer in temp {
            print!(",{}", computer);
        }
        println!("\n");
        //break;
    }


    /*let mut seen = Vec::new();
    
    for (computer1, connections1) in &graph {
        if seen.contains(computer1) { continue; }

        let mut all_connected = Vec::new();
        all_connected.push(computer1.clone());

        let mut todo = connections1.clone();

        while !todo.is_empty() {
            let next = todo.pop().unwrap();

            if all_connected.contains(&next) { continue; }

            all_connected.push(next.clone());

            let connections2 = graph.get(&next).unwrap();

            for connection in connections2 {
                todo.push(connection.clone());
            }
        }

        println!("{} {:?}", all_connected.len(), all_connected);

        for computer in all_connected {
            seen.push(computer.clone());
        }
    }*/

}