use std::io;

// this part could be easily done with the part 2 solution, just by changing what we total up, but kept this first version anyways

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    // unwrap and collect all the lines
    let lines = lines.map(|line| line.unwrap());
    let lines = lines.collect::<Vec<_>>();

    // grab all the comma separated towel descriptions
    let towels = lines[0].split(", ").collect::<Vec<_>>();

    let mut total = 0;

    // loop through all the remaining lines of the file (each containing a single design)
    for line in lines.iter().skip(2) {
        let mut tried = Vec::new();
        let mut todo = Vec::new();
        todo.push(0);

        // attempt to make all designs of each length, and keep track of how many ways it is possible
        'main_loop:
        while !todo.is_empty() {
            let current_index = todo.pop().unwrap();

            // if we've been here before, don't do the work again
            if tried.contains(&current_index) { continue; }
            tried.push(current_index);

            // loop through all the towel designs
            for towel in towels.clone().into_iter() {
                let end_index = current_index + towel.len();
                // see if this towel design matches for this spot in the desired pattern (and doesn't go off the edge of the pattern)
                if towel == &line[current_index..std::cmp::min(end_index, line.len())] {
                    let next_index = current_index + towel.len();

                    // if we've reached the end of the pattern, then it can be made, so count it, and exit
                    if next_index == line.len() {
                        total += 1;
                        break 'main_loop;
                    // otherwise, keep searching from here
                    } else {
                        todo.push(next_index);
                    }
                }
            }
        }
    }

    println!("TOTAL: {total}");
}
