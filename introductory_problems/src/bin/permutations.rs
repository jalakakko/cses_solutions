use std::io::*;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().unwrap();
    match n {
        n if n <= 3 && n != 1 => println!("NO SOLUTION"),
        _ => {
            let vec: Vec<i64> = (1..n+1).collect();
            let mut evens = vec![];
            let mut odds = vec![];
            for n in vec {
                if n % 2 == 0 {
                    evens.push(n);
                } else {
                    odds.push(n);
                }
            }
            evens.append(&mut odds);
            for i in evens {
                print!("{} ", i);
            }
        }
    }
}