use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(|x| x.unwrap().trim().parse().unwrap());
    let mut count = 0;
    let mut prev: Option<i64> = None;
    while let Some(line) = lines.next() {
        let val: i64 = line;
        if let Some(prev) = prev {
            if prev < val {
                count += 1;
            }
        }
        prev = Some(val);
    }
    println!("{}", count);
}
