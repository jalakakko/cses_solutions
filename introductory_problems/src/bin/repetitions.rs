use std::io::*;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let chars: Vec<char> = input.chars().collect(); 
    let mut count = 0;
    let mut highest = 0;
    let mut current = ' ';
    for c in chars { 
        match c {
            'A' | 'C' | 'G' | 'T' => { 
                if current != c {
                    count = 0;
                }
                count += 1;
                if count >= highest {
                    highest = count;
                }
                current = c;
            },
            _ => ()
        }
    }
    println!("{}", highest);
}