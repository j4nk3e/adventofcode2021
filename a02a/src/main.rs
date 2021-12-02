use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut x = 0;
    let mut y = 0;
    while let Some(line) = lines.next() {
        let re = Regex::new(r"^(\w+) (\d+)$").unwrap();
        let line = line.unwrap();
        let groups = re.captures(line.trim()).unwrap();
        let direction = groups.get(1).unwrap().as_str();
        let count: u64 = groups.get(2).unwrap().as_str().parse().unwrap();
        match direction {
            "forward" => x += count,
            "up" => y -= count,
            "down" => y += count,
            _ => panic!("unknown direction {}", direction),
        }
    }
    println!("{}", x * y);
}
