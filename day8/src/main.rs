use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;


fn main() {

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut v: Vec<String> = vec![];
    for l in lines {
        let line = l.unwrap();
        v.push(line);
    }


    let first = v.get(0).unwrap();
    let width = first.len() as u32;
    let height = v.len() as u32;


    let mut result = 0;
    for h in 0..height {
        for w in 0..width {
            let s = score(&v, h, w);
            if s > result {
                result = s;
            }


        }
    }

    println!("Result: {}", result);
}


fn score (input: &Vec<String>, h: u32, w: u32) -> u32 {
    let l = input.get(h as usize).unwrap();
    let horizontal = score_horizontal(&l, w);
    let vertical = score_vertical(input, h, w);
    return  horizontal * vertical;
}

fn score_horizontal(input: &String, wp: u32) -> u32 {
    let width = input.len() as u32;
    let tree = height_at_line(&input, wp);

        let mut ret1 = 0;

        if wp > 0 {
            for i in (0..wp).rev() {
                ret1 += 1;
                if height_at_line(&input,  i) >= tree {
                    break;
                }
            }
        }

        let mut ret2 = 0;

        if wp < width - 1 {
            for i in (wp + 1)..width {
                ret2 += 1;
                if height_at_line(&input, i) >= tree {
                    break;
                }
            }
        }

        return ret1 * ret2;
}

fn score_vertical(input: &Vec<String>, hp: u32, wp: u32) -> u32 {
    let height = input.len() as u32;
    let tree = height_at(&input, hp, wp);


    let mut ret1 = 0;

    if hp > 0 {
        for i in (0..hp).rev() {
            ret1 += 1;
            if height_at(&input,  i, wp) >= tree {
                break;
            }
        }
    }

    let mut ret2 = 0;
    if hp < height - 1 {
        for i in (hp + 1)..height {
            ret2 += 1;
            if height_at(&input,  i, wp) >= tree {
                break;
            }
        }
    }

    return ret1 * ret2;
}



fn height_at(input: &Vec<String>, h: u32, w: u32) -> u32 {
    let line = input.get(h as usize).unwrap();
    return height_at_line(line, w);
}

fn height_at_line(input: &String, y: u32) -> u32 {
    let char = input.chars().nth(y as usize).unwrap();
    return char.to_digit(10).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_score_horizontal() {
        assert_eq!(0, score_horizontal(&"33200330".to_string(), 0));
        assert_eq!(4, score_horizontal(&"33200330".to_string(), 5));
        assert_eq!(0, score_horizontal(&"33200330".to_string(), 7));
        assert_eq!(1, score_horizontal(&"33200330".to_string(), 3));
    }

    #[test]
    fn test_score_vertical() {

    let v: Vec<String> = vec![
            "303".to_string(),
            "252".to_string(),
            "622".to_string(),
            "682".to_string(),
            "632".to_string(),
            "652".to_string()
        ];

        assert_eq!(0, score_vertical(&v, 0, 1));
        assert_eq!(2, score_vertical(&v, 1, 1));
        assert_eq!(1, score_vertical(&v, 2, 1));
        assert_eq!(6, score_vertical(&v, 3, 1));
        assert_eq!(1, score_vertical(&v, 4, 1));
        assert_eq!(0, score_vertical(&v, 5, 1));

        let v2: Vec<String> = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string()
        ];

        assert_eq!(2, score_vertical(&v2, 1, 2));
    }


#[test]
fn test_score() {
    let v: Vec<String> = vec![
        "30373".to_string(),
        "25512".to_string(),
        "65332".to_string(),
        "33549".to_string(),
        "35390".to_string()
    ];

    assert_eq!(4, score(&v, 1, 2));
    assert_eq!(8, score(&v, 3, 2));
}


}