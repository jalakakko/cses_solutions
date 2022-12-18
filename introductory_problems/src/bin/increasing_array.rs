use std::io::*;

fn main() {
    let mut arr_size = String::new();
    stdin().read_line(&mut arr_size);
    let arr_size: usize = arr_size.trim().parse().unwrap();

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input);
        let mut input_vec: Vec<i64> = input.split(" ").map(|x| x.trim().parse::<i64>().unwrap()).collect();
        match input_vec.len() {
            input_size if input_size != arr_size => println!("Input as many numbers as the size of the array"),
            _ => {
                let mut c: i64 = 0;
                let mut p: i64 = 0;

                for n in input_vec {
                    if n < p {
                        c = c + (p - n);
                    } else {
                        p = n;
                    }
                }
                println!("{}", c);
                break;
            }
        } 
    }
}