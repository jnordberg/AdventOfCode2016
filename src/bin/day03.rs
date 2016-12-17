use std::io::{self};
use std::io::BufRead;

fn is_valid_triangle(sides: &Vec<i32>) -> bool {
    for (i, side) in sides.iter().enumerate() {
        let other = sides[(i + 1) % 3] + sides[(i + 2) % 3];
        if side >= &other {
            return false;
        }
    }
    return true;
}

fn main() {
    let stdin = io::stdin();

    let mut num_valid = 0;
    let mut num_valid_col = 0;

    let mut c1 = Vec::new();
    let mut c2 = Vec::new();
    let mut c3 = Vec::new();

    for (line_idx, line) in stdin.lock().lines().enumerate() {
        let unwrapped_line = line.unwrap();
        let sides = unwrapped_line.split(' ').into_iter()
            .filter(|&v|v.len() > 0)
            .map(|v|v.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        c1.push(sides[0]);
        c2.push(sides[1]);
        c3.push(sides[2]);

        if (line_idx % 3) == 2 {
            if is_valid_triangle(&c1) { num_valid_col += 1; } c1.clear();
            if is_valid_triangle(&c2) { num_valid_col += 1; } c2.clear();
            if is_valid_triangle(&c3) { num_valid_col += 1; } c3.clear();
        }

        if is_valid_triangle(&sides) {
            num_valid += 1;
        }
    }

    println!("Valid triangles: {}", num_valid);
    println!("Valid column triangles: {}", num_valid_col);
}
