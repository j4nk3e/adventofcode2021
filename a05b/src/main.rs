use regex::Regex;
use std::{
    cmp,
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut field: HashMap<isize, HashMap<isize, isize>> = HashMap::new();
    while let Some(line) = lines.next() {
        let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        let line = line.unwrap();
        let groups = re.captures(line.trim()).unwrap();
        let x1: isize = groups.get(1).unwrap().as_str().parse().unwrap();
        let y1: isize = groups.get(2).unwrap().as_str().parse().unwrap();
        let x2: isize = groups.get(3).unwrap().as_str().parse().unwrap();
        let y2: isize = groups.get(4).unwrap().as_str().parse().unwrap();
        let dx = (x1 - x2).abs();
        let dy = (y1 - y2).abs();
        assert!(dx == dy || dx == 0 || dy == 0);
        let mut y = y1.clone();
        let mut x = x1.clone();
        let steps = cmp::max(dx, dy) + 1;
        for _ in 0..steps {
            let q = field.entry(y).or_insert(HashMap::new());
            let f = q.entry(x).or_default();
            *f += 1;
            if dy > 0 {
                y += if y2 > y1 { 1 } else { -1 }
            }
            if dx > 0 {
                x += if x2 > x1 { 1 } else { -1 }
            }
        }
    }
    let mut count = 0usize;
    for y in field.values() {
        for x in y.values() {
            if x > &1 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
