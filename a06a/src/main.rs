use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut fish: VecDeque<usize> = VecDeque::with_capacity(9);
    for _ in 0..9 {
        fish.push_back(0);
    }
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let numbers: Vec<usize> = line.split(",").map(|s| s.parse().unwrap()).collect();

        for n in numbers.iter() {
            let q = fish.get_mut(*n).unwrap();
            *q = *q + 1;
        }
        for _ in 0..256 {
            let n = fish.pop_front().unwrap();
            fish.push_back(n);
            let q = fish.get_mut(6).unwrap();
            *q += n;
        }
        let sum: usize = fish.iter().sum();
        println!("{}", sum);
    }
}
