use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut counts: Vec<i64> = Vec::new();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if counts.is_empty() {
            for _ in 0..line.len() {
                counts.push(0);
            }
        }
        let mut c_iter = counts.iter_mut();
        for (_, char) in line.chars().enumerate() {
            let item = c_iter.next().unwrap();
            *item += match char {
                '0' => -1,
                '1' => 1,
                _ => panic!("invalid char"),
            };
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in counts.iter() {
        if i > &0 {
            gamma = (gamma << 1) + 1;
            epsilon = epsilon << 1;
        } else {
            gamma = gamma << 1 ;
            epsilon = (epsilon << 1) + 1;
        }
    }
    println!(
        "{} | {}",
        counts
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(","),
        gamma * epsilon
    );
}
