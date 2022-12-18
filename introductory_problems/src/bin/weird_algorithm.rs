use std::io::*;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut y: u64 = input.trim().parse().unwrap();
    while y != 1 {
        if y % 2 == 0 {
            y = y / 2;
        } else {
            y = y * 3 + 1
        }
        print!("{} ", y);
    }
}