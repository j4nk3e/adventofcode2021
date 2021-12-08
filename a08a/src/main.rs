use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let allowed = [2, 3, 4, 7];
    let mut count = 0;
    for line in lines {
        let re = Regex::new(r"^(.*) \| (.*)$").unwrap();
        let line = line.unwrap();
        let groups = re.captures(line.trim()).unwrap();
        count += groups
            .get(2)
            .unwrap()
            .as_str()
            .split_whitespace()
            .filter(|s| allowed.contains(&s.len()))
            .count();
    }
    println!("{}", count);
}
