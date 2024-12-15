use std::io;
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    // unwrap all the lines
    let lines = lines.map(|line| line.unwrap());

    // concat all the strings
    let all = lines.collect::<String>();

    // extract all the numbers via a regex
    let all_numbers = Regex::new(r"-?\d+").unwrap().captures_iter(&all)
                // convert them all to i128s
                .map(|c| c[0].parse::<i128>().unwrap())
                // collect them into a vector for easy retrieval
                .collect::<Vec<_>>();

    //println!("{all_numbers:?}");

    let mut total = 0;

    // group the numbers into groups of 6 elements
    let chunks = all_numbers.chunks(4);

    let width = 101; //11;
    let height = 103; //7;

    let mut robots = Vec::new();

    for chunk in chunks {
        // read the numbers from the chunk
        let [px, py, dx, dy] = chunk else { panic!("not the right amount of numbers in input") };

        robots.push((*px, *py, *dx, *dy));
    }

    fn print_robots(robots: &Vec<(i128, i128, i128, i128)>, width: i128, height: i128) {
        for j in 0..height {
            for i in 0..width {
                //let at_here = 
                //    robots.clone().iter().filter(|(x, y, _dx, _dy)| *x == i && *y == j);
                //let num = at_here.collect::<Vec<_>>().len();
    
                let mut num = 0;
                for robot in robots.clone() {
                    let (x, y, _dx, _dy) = robot;
                    if x == i && y == j {
                        num += 1;
                    }
                }
    
                if num == 0 {
                    print!(".");
                } else {
                    print!("{}", num);
                }
            }
            println!();
        }
        println!();
    }

    //print_robots(&robots, width, height);

    let steps = 100;

    for ii in 0..steps {
        for i in 0..robots.len() {
            let (x, y, dx, dy) = robots[i];
            //println!("{x} {y} {dx} {dy}");
            robots[i] = (
                (x + width + dx) % width,
                (y + height + dy) % height,
                dx,
                dy
            );
            let (x, y, dx, dy) = robots[i];
            //println!("-> {x} {y} {dx} {dy}");
        }

        //println!("STEP: {ii}");
        //print_robots(&robots, width, height);
    }

    //print_robots(&robots, width, height);

    let mut quads = [0, 0, 0, 0];

    for i in 0..robots.len() {
        let (x, y, dx, dy) = robots[i];

        let mut index = 0;
        if x < width / 2 {
        } else if x == width / 2 {
            continue;
        }else if x > width / 2 {
            index += 2;
        }

        if y < height / 2 {
        } else if y == height / 2 {
            continue;
        }else if y > height / 2 {
            index += 1;
        }

        quads[index] += 1;
    }

    println!("{quads:?}");
    
    total = quads[0] * quads[1] * quads[2] * quads[3];

    println!("TOTAL: {}", total);
}

// 91976256 too low
// 222381250 too low
// 225109221 too low <- up to and including this point, I wasn't parsing -ve numbers from the input
// 232253028 
