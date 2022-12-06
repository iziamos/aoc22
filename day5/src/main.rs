
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let mut vec: Vec<Vec<char>> = vec![
        vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]
     ];
    // for i in 0..9 {
    //     vec.push(Vec::new);
    // }


    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let s = line.unwrap();

        if s.len() == 0 {
            continue;
        }

        let chars  = s.as_bytes();

        let first = chars[0] as char;

        if first == '[' {
            for i in 0..9 {
                let index = i * 4 + 1;
                let c = chars[index] as char;
                if c == ' ' {
                    continue;
                }
                vec[i].insert(0, c)
            }
            continue;
        }

        if first == ' ' {
            continue;
        }

        if first == 'm' {
            // let slice = &s[5..];
            let mut split = s.split_whitespace();

            split.next(); // move
            let count = split.next().unwrap().parse::<i32>().unwrap();
            split.next(); //from
            let source = split.next().unwrap().parse::<i32>().unwrap(); // src
            split.next(); //to
            let dest = split.next().unwrap().parse::<i32>().unwrap();


            for _ in 0..count {
                match vec[ (source - 1 ) as usize].pop() {
                    None => continue,
                    Some(c) => vec[(dest - 1) as usize].push(c)
                }
            }

            // println!("count:{}, source:{}, dest{}", count, source, dest);
        }
    }


    println!("{} ", vec[0].pop().unwrap());
    println!("{} ", vec[1].pop().unwrap());
    println!("{} ", vec[2].pop().unwrap());
    println!("{} ", vec[3].pop().unwrap());
    println!("{} ", vec[4].pop().unwrap());
    println!("{} ", vec[5].pop().unwrap());
    println!("{} ", vec[6].pop().unwrap());
    println!("{} ", vec[7].pop().unwrap());
    println!("{} ", vec[8].pop().unwrap());

    Ok(())
}
