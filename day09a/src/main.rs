use std::io;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let line = lines.next().unwrap().unwrap();
    let line = line.chars().collect::<Vec<char>>();

    let mut disc = Vec::new();
    
    for (x, c) in line.into_iter().enumerate() {
        for _i in 0..c.to_digit(10).unwrap() {
            disc.push(if x % 2 == 0 {
                Some(x / 2)
            } else {
                None
            });
        }
    }

    loop {
        let hole_to_fill = disc.iter().position(|&item| item == None);

        if let Some(hole_to_fill) = hole_to_fill {
            //println!("{hole_to_fill}");

            disc[hole_to_fill] = disc.pop().unwrap();
    
            while disc[disc.len() - 1] == None {
                disc.pop();
            }
        } else {
            break;
        }
    }

    //println!("{:?}", disc);

    // calculate total
    let total = disc.iter()
        .enumerate()
        .map(|(i, x)| i * x.unwrap());
    let total = total.sum::<usize>();

    println!("{}", total);
}
