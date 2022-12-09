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
            if is_invisible(&v, h, w) {
                result += 1;
            }
        }
    }

    println!("Result: {}", (height * width) - result);
}


fn is_invisible(input: &Vec<String>, h: u32, w: u32) -> bool {
    let l = input.get(h as usize).unwrap();
    return is_invisible_horizontal(l, w) && is_invisible_vertical(input, h, w);
}

fn is_invisible_horizontal(input: &String, wp: u32) -> bool {
    let width = input.len() as u32;
    let tree = height_at_line(&input, wp);

        if (wp == width - 1) || wp == 0 {
            return false;
        }

        let mut ret: bool = false;

        for i in 0..wp {
            if height_at_line(&input,  i) >= tree {
                ret = true;
            }
        }


        for i in (wp + 1)..width {
            if height_at_line(&input, i) >= tree {
                return ret & true;
            }
        }

        return false;
}

fn is_invisible_vertical(input: &Vec<String>, hp: u32, wp: u32) -> bool {
    let height = input.len() as u32;
    let tree = height_at(&input, hp, wp);

    if (hp == height - 1) || hp == 0 {
        return false;
    }

    let mut ret: bool = false;

    for i in (0..hp).rev() {
        if height_at(&input,  i, wp) >= tree {
            ret = true;
        }
    }

    for i in (hp + 1)..height {
        if height_at(&input,  i, wp) >= tree {
            return ret & true;
        }
    }

    return false;
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
    fn test_is_invisible_horizontal() {
        assert_eq!(false, is_invisible_horizontal(&"12351".to_string(), 0));
        assert_eq!(false, is_invisible_horizontal(&"12354".to_string(), 1));
        assert_eq!(true, is_invisible_horizontal(&"32354".to_string(), 2));
        assert_eq!(true, is_invisible_horizontal(&"32054".to_string(), 2));
        assert_eq!(false, is_invisible_horizontal(&"12354".to_string(), 3));
        assert_eq!(false, is_invisible_horizontal(&"123549".to_string(), 3));
        assert_eq!(false, is_invisible_horizontal(&"12351".to_string(), 4));

        assert_eq!(false, is_invisible_horizontal(&"11111".to_string(), 0));
        assert_eq!(true, is_invisible_horizontal(&"11111".to_string(), 1));
        assert_eq!(true, is_invisible_horizontal(&"11111".to_string(), 2));
        assert_eq!(true, is_invisible_horizontal(&"11111".to_string(), 3));
        assert_eq!(false, is_invisible_horizontal(&"11111".to_string(), 4));

        assert_eq!(false, is_invisible_horizontal(&"10001".to_string(), 0));
        assert_eq!(true, is_invisible_horizontal(&"10001".to_string(), 1));
        assert_eq!(true, is_invisible_horizontal(&"10001".to_string(), 2));
        assert_eq!(true, is_invisible_horizontal(&"10001".to_string(), 3));
        assert_eq!(false, is_invisible_horizontal(&"10001".to_string(), 4));

        assert_eq!(false, is_invisible_horizontal(&"30373".to_string(), 0));
        assert_eq!(true, is_invisible_horizontal(&"30373".to_string(), 1));
        assert_eq!(true, is_invisible_horizontal(&"30373".to_string(), 2));
        assert_eq!(false, is_invisible_horizontal(&"30373".to_string(), 3));
        assert_eq!(false, is_invisible_horizontal(&"30373".to_string(), 4));
    }

    #[test]
    fn test_height_at_line() {
        assert_eq!(1, height_at_line(&"12351".to_string(), 0));
        assert_eq!(2, height_at_line(&"12351".to_string(), 1));
        assert_eq!(3, height_at_line(&"12351".to_string(), 2));
        assert_eq!(6, height_at_line(&"12361".to_string(), 3));
        assert_eq!(1, height_at_line(&"12351".to_string(), 4));
    }

    #[test]
    fn test_height_at() {

        let v: Vec<String> = vec![
            "303".to_string(),
            "252".to_string(),
            "652".to_string()
        ];


        assert_eq!(3, height_at(&v, 0, 0));
        assert_eq!(0, height_at(&v, 0, 1));
        assert_eq!(3, height_at(&v, 0, 2));

        assert_eq!(2, height_at(&v, 1, 0));
        assert_eq!(5, height_at(&v, 1, 1));
        assert_eq!(2, height_at(&v, 1, 2));

        assert_eq!(6, height_at(&v, 2, 0));
        assert_eq!(5, height_at(&v, 2, 1));
        assert_eq!(2, height_at(&v, 2, 2));
    }

    #[test]
    fn test_is_invisible_vertical() {

        let v: Vec<String> = vec![
            "303".to_string(),
            "252".to_string(),
            "622".to_string(),
            "682".to_string(),
            "632".to_string(),
            "652".to_string()
        ];


        assert_eq!(false, is_invisible_vertical(&v, 0, 1));
        assert_eq!(false, is_invisible_vertical(&v, 1, 1));
        assert_eq!(true,  is_invisible_vertical(&v, 2, 1));
        assert_eq!(false, is_invisible_vertical(&v, 3, 1));
        assert_eq!(true,  is_invisible_vertical(&v, 4, 1));
        assert_eq!(false, is_invisible_vertical(&v, 5, 1));

    }

    #[test]
    fn test_is_invisible() {

        let v: Vec<String> = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string()
        ];

        assert_eq!(false, is_invisible(&v, 0, 0)); //h //w
        assert_eq!(false, is_invisible(&v, 1, 0));
        assert_eq!(false, is_invisible(&v, 2, 0));
        assert_eq!(false, is_invisible(&v, 3, 0));
        assert_eq!(false, is_invisible(&v, 4, 0));


        assert_eq!(false, is_invisible(&v, 0, 1));
        assert_eq!(false, is_invisible(&v, 1, 1));
        assert_eq!(false, is_invisible(&v, 2, 1));
        assert_eq!(true, is_invisible(&v, 3, 1));
        assert_eq!(false, is_invisible(&v, 4, 1));


        assert_eq!(false, is_invisible(&v, 0, 2));
        assert_eq!(false, is_invisible(&v, 1, 2));
        assert_eq!(true, is_invisible(&v, 2, 2));
        assert_eq!(false, is_invisible(&v, 3, 2));
        assert_eq!(false, is_invisible(&v, 4, 2));


        assert_eq!(false, is_invisible(&v, 0, 3));
        assert_eq!(true, is_invisible(&v, 1, 3));
        assert_eq!(false, is_invisible(&v, 2, 3));
        assert_eq!(true, is_invisible(&v, 3, 3));
        assert_eq!(false, is_invisible(&v, 4, 3));


        assert_eq!(false, is_invisible(&v, 0, 4));
        assert_eq!(false, is_invisible(&v, 1, 4));
        assert_eq!(false, is_invisible(&v, 2, 4));
        assert_eq!(false, is_invisible(&v, 3, 4));
        assert_eq!(false, is_invisible(&v, 4, 4));
    }

}