use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    println!(
        "{}",
        filter(lines.clone(), true) * filter(lines.clone(), false)
    );
}

fn filter(mut lines: Vec<String>, eq: bool) -> i64 {
    let mut i = 0;
    while lines.len() > 1 {
        lines.sort();
        let mid = lines.get(lines.len() / 2).unwrap();
        let x = mid.chars().enumerate().nth(i);
        lines = lines
            .iter()
            .filter(|l| (l.chars().enumerate().nth(i) == x) == eq)
            .cloned()
            .collect();
        i += 1;
    }
    return i64::from_str_radix(lines.get(0).unwrap(), 2).unwrap();
}
