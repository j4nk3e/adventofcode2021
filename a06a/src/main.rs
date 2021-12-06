use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut fish: VecDeque<usize> = VecDeque::from(vec![0; 9]);
    for n in lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>()
    {
        *fish.get_mut(n).unwrap() += 1;
    }
    for _ in 0..256 {
        *fish.get_mut(7).unwrap() += *fish.get(0).unwrap();
        fish.rotate_left(1);
    }
    let sum: usize = fish.iter().sum();
    println!("{}", sum);
}
