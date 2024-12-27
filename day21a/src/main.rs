//use core::num;
use std::io;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    // unwrap and collect all the lines
    let lines = lines.map(|line| line.unwrap());
    //let lines = lines.collect::<Vec<_>>();

    let mut total = 0;

    // loop through all the remaining lines of the file (each containing a single design)
    for line in lines {
        // initialise our dynamic programming style array to have 1 way to make a zero length design, and 0 ways to make all the other lengths
        let mut subtotal= 0;

        println!("{line}");

        let numbers_only = line.chars().filter(|c|c.is_ascii_digit());

        let keypad = vec!("#####", "#789#", "#456#", "#123#", "##0A#", "#####");
        //let keypad = keypad.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
        let mut pos = (3, 4);

        /*let search = | keypad, (sx, sy), (ex, ey), sequence_so_far | {
            let mut ret = Vec::new();

            /*if keypad[sy][sx] == '#' {
                return ret;
            }*/

            if ey < sy && keypad[sy - 1][sx] != '#' {
                ret.push(((sx, sy - 1), sequence_so_far + "^"));
            } else if ey > sy && keypad[sy + 1][sx] != '#' {
                ret.push(((sx, sy + 1), sequence_so_far + "v"));
            }

            if ex < sx && keypad[sy][sx - 1] != '#' {
                ret.push(((sx - 1, sy), sequence_so_far + "<"));
            } else if ex > sx && keypad[sy][sx + 1] != '#' {
                ret.push(((sx + 1, sy), sequence_so_far + ">"));
            }

            return ret;
        };*/

        let mut sequence = String::new();
        for c in line.chars() {
            //println!("searching for {c}");
            let dest = keypad.iter().enumerate().find_map(|(y, row)| row.find(c).and_then(|x| Some((x, y))));
            let (dx, dy) = dest.unwrap();
            //println!("{dx} {dy}");
            let (mut x, mut y) = pos;

            if keypad[y].chars().collect::<Vec<char>>()[dx] == '#' {
                while x != dx || y != dy {
                    if y < dy && keypad[dy].chars().collect::<Vec<char>>()[x] != '#' {
                        y += 1;
                        sequence += "v";
                    } else if y > dy && keypad[dy].chars().collect::<Vec<char>>()[x] != '#'{
                        y -= 1;
                        sequence += "^";
                    } else if x > dx && keypad[y].chars().collect::<Vec<char>>()[dx] != '#' {
                        x -= 1;
                        sequence += "<";
                    } else if x < dx && keypad[y].chars().collect::<Vec<char>>()[dx] != '#' {
                        x += 1;
                        sequence += ">";
                    }
                }
            } else {
                while x != dx || y != dy {
                    if x > dx && keypad[y].chars().collect::<Vec<char>>()[dx] != '#' {
                        x -= 1;
                        sequence += "<";
                    } else if x < dx && keypad[y].chars().collect::<Vec<char>>()[dx] != '#' {
                        x += 1;
                        sequence += ">";
                    } else if y < dy && keypad[dy].chars().collect::<Vec<char>>()[x] != '#' {
                        y += 1;
                        sequence += "v";
                    } else if y > dy && keypad[dy].chars().collect::<Vec<char>>()[x] != '#'{
                        y -= 1;
                        sequence += "^";
                    }                
                }
            }

            sequence += "A";
            pos = (x, y);
        }
        println!("{sequence}");

        let line = sequence;
        let keypad = vec!("#####", "##^A#", "#<v>#", "#####");
        let mut pos = (3, 1);

        let mut sequence = String::new();
        for c in line.chars() {
            //println!("searching for {c}");
            let dest = keypad.iter().enumerate().find_map(|(y, row)| row.find(c).and_then(|x| Some((x, y))));
            let (dx, dy) = dest.unwrap();
            //println!("{dx} {dy}");
            let (mut x, mut y) = pos;
            if keypad[y].chars().collect::<Vec<char>>()[dx] == '#' {
                while x != dx || y != dy {
                    if y < dy && keypad[dy].chars().collect::<Vec<char>>()[x] != '#' {
                        y += 1;
                        sequence += "v";
                    } else if y > dy && keypad[dy].chars().collect::<Vec<char>>()[x] != '#'{
                        y -= 1;
                        sequence += "^";
                    } else if x > dx && keypad[y].chars().collect::<Vec<char>>()[dx] != '#' {
                        x -= 1;
                        sequence += "<";
                    } else if x < dx && keypad[y].chars().collect::<Vec<char>>()[dx] != '#' {
                        x += 1;
                        sequence += ">";
                    }
                }
            } else {
                while x != dx || y != dy {
                    if x > dx && keypad[y].chars().collect::<Vec<char>>()[dx] != '#' {
                        x -= 1;
                        sequence += "<";
                    } else if x < dx && keypad[y].chars().collect::<Vec<char>>()[dx] != '#' {
                        x += 1;
                        sequence += ">";
                    } else if y < dy && keypad[dy].chars().collect::<Vec<char>>()[x] != '#' {
                        y += 1;
                        sequence += "v";
                    } else if y > dy && keypad[dy].chars().collect::<Vec<char>>()[x] != '#'{
                        y -= 1;
                        sequence += "^";
                    }                
                }
            }
            sequence += "A";
            pos = (x, y);
        }
        println!("{sequence}");

        let line = sequence;
        let keypad = vec!("#####", "##^A#", "#<v>#", "#####");
        let mut pos = (3, 1);

        let mut sequence = String::new();
        for c in line.chars() {
            //println!("searching for {c}");
            let dest = keypad.iter().enumerate().find_map(|(y, row)| row.find(c).and_then(|x| Some((x, y))));
            let (dx, dy) = dest.unwrap();
            //println!("{dx} {dy}");
            let (mut x, mut y) = pos;
            if keypad[y].chars().collect::<Vec<char>>()[dx] == '#' {
                while x != dx || y != dy {
                    if y < dy && keypad[dy].chars().collect::<Vec<char>>()[x] != '#' {
                        y += 1;
                        sequence += "v";
                    } else if y > dy && keypad[dy].chars().collect::<Vec<char>>()[x] != '#'{
                        y -= 1;
                        sequence += "^";
                    } else if x > dx && keypad[y].chars().collect::<Vec<char>>()[dx] != '#' {
                        x -= 1;
                        sequence += "<";
                    } else if x < dx && keypad[y].chars().collect::<Vec<char>>()[dx] != '#' {
                        x += 1;
                        sequence += ">";
                    }
                }
            } else {
                while x != dx || y != dy {
                    if x > dx && keypad[y].chars().collect::<Vec<char>>()[dx] != '#' {
                        x -= 1;
                        sequence += "<";
                    } else if x < dx && keypad[y].chars().collect::<Vec<char>>()[dx] != '#' {
                        x += 1;
                        sequence += ">";
                    } else if y < dy && keypad[dy].chars().collect::<Vec<char>>()[x] != '#' {
                        y += 1;
                        sequence += "v";
                    } else if y > dy && keypad[dy].chars().collect::<Vec<char>>()[x] != '#'{
                        y -= 1;
                        sequence += "^";
                    }                
                }
            }
            sequence += "A";
            pos = (x, y);
        }
        println!("{sequence}");

        let numbers_only = numbers_only.collect::<String>().parse::<usize>().unwrap();
        subtotal = sequence.len() * numbers_only;

        println!("{} * {} = {} ", sequence.len(), numbers_only, subtotal);

        // number of ways we can make this pattern is stored in the last position (if zero, i.e. a failure, add it on anyway)
        total += subtotal;
    }

    println!("TOTAL: {total}");
}

// 029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
//       <vA<AA>>^AvAA<^A>Av<<A>>^AvA^A<vA>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A
// 980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
//       v<<A>>^AAAvA^A<vA<AA>>^AvAA<^A>Av<<A>A>^AAAvA<^A>A<vA>^A<A>A
// 179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
//       v<<A>>^A<vA<A>>^AAvAA<^A>Av<<A>>^AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A
// 456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
//       v<<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>Av<<A>A>^AAvA<^A>A
// 379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
//       v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A



// 143536 too high