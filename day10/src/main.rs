use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut c = 1;

    let mut tally = 1;
    for l in lines {
        let line = l.unwrap();
        if line == "noop" {
            println!("Cycle{} / Total{}", c, tally);
            c += 1;
            continue;
        }

        let mut split = line.split_ascii_whitespace();
        let second = split.nth(1).unwrap();
        let p: i32 = second.to_string().parse().unwrap();

        println!("Cycle{} / Total{}", c, tally);
        c += 1;
        println!("Cycle{} / Total{}",  c, tally);
        tally += p;
        c += 1;
    }

}
