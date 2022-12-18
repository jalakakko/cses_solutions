use std::io::*;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let x: u64 = input.trim().parse().unwrap();

    let mut y = String::new();
    stdin().read_line(&mut y).unwrap();
    let mut y: Vec<u64> = y.split(' ').map(|x| x.trim().parse::<u64>().unwrap()).collect();
    y.sort();
    y.push(0);

    let mut j = 0;
    let nums: Vec<u64> = (1..x+1).collect();
    for i in &y {
        if *i != nums[j] {
            println!("{}", nums[j]);
            break;
        }
        j += 1;
    }
} 