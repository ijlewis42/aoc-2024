use std::io;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    let mut total = 0;

    // non-idiomatic approach to reading all the input
    for line in lines {
        let line = line.unwrap();
        let values = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        //println!("{:?}", values);

        // pair each adjacent value
        let adjacents = values.windows(2);
        //println!("{:?}", adjacents.clone().collect::<Vec<_>>());

        // find the difference between each adjacent value
        let diff = adjacents.map(|a| a[0] - a[1]);
        // more idiomatic, but uglier I think
        /*let diff = adjacents.map(|a| match a {
            [x, y] => x - y,
            _ => panic!("Impossible")
        });*/

        //println!("{:?}", diff.clone().collect::<Vec<_>>());

        // check that all values have the same sign (i.e. all positive, or all negative numbers)
        // I feel there should be some more elegant way of getting the first element here, or not needing to
        let first = diff.clone().next().unwrap().signum();      // OR diff.clone().collect::<Vec<_>>()[0].signum();
        let same_direction = diff.clone().all(|x| x.signum() == first);
        //println!("{same_direction}");

        // gross alternate approach
        /*let same_direction = diff.clone()
        .map(|x| x.signum())
        .collect::<Vec<_>>()
        .windows(2)
        .all(|a| match a { 
            [x, y] => x == y, 
            _ => panic!("Impossible")
        });*/

        // check all values have small increments (or decrements)
        let small_diff = diff.clone().all(|x| x.abs() >= 1 && x.abs() <= 3);
        //println!("{small_diff}");

        // if pass both tests, woohoo
        if same_direction && small_diff {
            total += 1;
        }
    }

    println!("{}", total);
}
