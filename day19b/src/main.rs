use std::io;

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
        // initialise our dynamic programming style array to have 1 way to make a zero length design, and 0 ways to make all the other lengths
        let mut subtotals: Vec<u128> = Vec::new();
        subtotals.push(1);
        subtotals.extend((0..line.len()).map(|_|0));

        // attempt to make all designs of each length, and keep track of how many ways it is possible
        for current_index in 0..line.len() {
            // if no ways to make the current length, not going to be able to make longer lengths _from_ here, so skip it
            if subtotals[current_index] > 0 {
                // loop through all the towel designs
                for towel in towels.clone().into_iter() {
                    let end_index = current_index + towel.len();
                    // see if this towel design matches for this spot in the desired pattern (and doesn't go off the edge of the pattern)
                    if towel == &line[current_index..std::cmp::min(end_index, line.len())] {
                        // if so, add the amount of ways we could make this pattern
                        subtotals[end_index] += subtotals[current_index];
                    }
                }
            }
        }

        // number of ways we can make this pattern is stored in the last position (if zero, i.e. a failure, add it on anyway)
        total += subtotals[subtotals.len() - 1];
    }

    println!("TOTAL: {total}");
}