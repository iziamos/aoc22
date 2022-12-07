use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

fn main() {

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let line = lines.next().unwrap().unwrap();

    let length = line.len();
    for i in 0..length {

        let segment = &line[i..i + 14];
        let u = chars_are_unique(segment);

        println!("{} {} {}", i + 14, segment, chars_are_unique(segment));
        if u {
            break;
        }
    }
}

fn chars_are_unique(s: &str) -> bool {
    let mut c : HashSet<char> = HashSet::new();
    for b in s.chars() {
        c.insert(b);
    }
    return c.len() == 14;
}