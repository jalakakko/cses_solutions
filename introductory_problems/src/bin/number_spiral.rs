use std::{io::*, ops::Index};

fn main() {
    let mut tests = String::new();
    stdin().read_line(&mut tests).unwrap();
    let mut tests: u64 = tests.trim().parse().unwrap();

    let mut coordinates: Vec<(u64, u64)> = vec![];
    while tests != 0 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut input_split = input.split(' ');
        let crd: (u64, u64) = (
            input_split.next().unwrap().trim().parse().unwrap(),
            input_split.next().unwrap().trim().parse().unwrap()
        );
        coordinates.push(crd);
        tests -= 1;
    }
    for c in coordinates {
        let y = c.0;
        let x = c.1;
        let max = std::cmp::max(y, x);
        let mut grid: u64 = 1;
        let mut multiplier: u64 = 0;
        
        for _ in 0..max {
            grid = grid + multiplier;
            multiplier += 2;
        }
        if y == 1 && x == 1 { 
            grid = 1;
            println!("{}", grid);
            continue;
        }
        if y == max {
            if y % 2 == 0 {
                grid = grid + (y-1) - (x-1)
            } else {
                grid = grid - (y-1) + (x-1)
            }
        } else {
            if x % 2 == 0 {
                grid = grid - (x-1) + (y-1)
            } else {
                grid = grid + (x-1) - (y-1)
            }
        }
        println!("{}", grid)
    }
}
