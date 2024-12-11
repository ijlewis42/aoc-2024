use std::io;
use std::time::Instant;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let line = lines.next().unwrap().unwrap();

    let mut values = line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();

    for blink in 0..25 {
        print!("{blink}: ");

        let start = Instant::now();

        let mut i = 0;
        while i < values.len() {
            let current = values[i];
    
            if current == 0 {
                //println!("case 0");
                values[i] = 1;
                i += 1;
            } else if current.to_string().len() % 2 == 0 {
                //println!("case EVEN digits");
                let s = current.to_string();
                let left = s[0..s.len() / 2].parse::<u64>();
                let right = s[s.len() / 2..s.len()].parse::<u64>();
                //println!("{s} {left:?} {right:?}");
                values[i] = left.unwrap();
                values.insert(i + 1, right.unwrap());
                i += 2;
            } else {
                //println!("case OTHER");
                values[i] = current * 2024;
                i += 1;
            }
        }
        let end = Instant::now();
        //println!("{:?}", values);
        println!("{} in {:?}", values.len(), end.duration_since(start));
    }

    println!("{}", values.len());


}
