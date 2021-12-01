use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut count = 0;
    let mut deq: VecDeque<u64> = VecDeque::new();
    while let Some(line) = lines.next() {
        let val: u64 = line.unwrap().trim().parse().unwrap();
        let mut prev: Option<u64> = None;
        if deq.len() == 3 {
            prev = Some(deq.iter().sum());
        }

        deq.push_back(val);
        if deq.len() > 3 {
            deq.pop_front();
        }

        if deq.len() == 3 {
            let next: u64 = deq.iter().sum();
            if let Some(prev) = prev {
                if prev < next {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
