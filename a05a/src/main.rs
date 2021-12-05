use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut h_lines: HashMap<usize, Vec<HLine>> = HashMap::new();
    let mut v_lines: HashMap<usize, Vec<HLine>> = HashMap::new();
    let mut max_x = 0usize;
    let mut max_y = 0usize;
    while let Some(line) = lines.next() {
        let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        let line = line.unwrap();
        let groups = re.captures(line.trim()).unwrap();
        let x1 = groups.get(1).unwrap().as_str().parse().unwrap();
        let y1 = groups.get(2).unwrap().as_str().parse().unwrap();
        let x2 = groups.get(3).unwrap().as_str().parse().unwrap();
        let y2 = groups.get(4).unwrap().as_str().parse().unwrap();
        if x1 == x2 {
            max_x = cmp::max(max_x, x1);
            max_y = cmp::max(cmp::max(max_y, y1), y2);
            let l = v_lines.entry(x1).or_insert(Vec::new());
            l.push(HLine {
                start: if y1 < y2 { y1 } else { y2 },
                end: if y1 < y2 { y2 } else { y1 },
            });
        } else if y1 == y2 {
            max_y = cmp::max(max_y, y1);
            max_x = cmp::max(cmp::max(max_x, x1), x2);
            let l = h_lines.entry(y1).or_insert(Vec::new());
            l.push(HLine {
                start: if x1 < x2 { x1 } else { x2 },
                end: if x1 < x2 { x2 } else { x1 },
            });
        }
    }
    max_x += 1;
    max_y += 1;
    let mut field: Vec<Vec<usize>> = Vec::with_capacity(max_y);
    for _ in 0..max_y {
        let mut q = Vec::with_capacity(max_x);
        for _ in 0..max_x {
            q.push(0);
        }
        field.push(q);
    }
    for (y, c) in h_lines.iter() {
        for l in c.iter() {
            for x in l.start..l.end + 1 {
                let q = field.get_mut(*y).unwrap();
                let r = q.get_mut(x).unwrap();
                *r += 1;
            }
        }
    }
    for (x, c) in v_lines.iter() {
        for l in c.iter() {
            for y in l.start..l.end + 1 {
                let q = field.get_mut(y).unwrap();
                let r = q.get_mut(*x).unwrap();
                *r += 1;
            }
        }
    }
    let mut count = 0usize;
    for y in field.iter() {
        for x in y.iter() {
            if x > &1 {
                count += 1;
            }
        }
    }
    println!("h: {}, v: {} | {}", max_x, max_y, count);
}

struct HLine {
    start: usize,
    end: usize,
}
