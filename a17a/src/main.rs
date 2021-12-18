use std::io::{self, BufRead};

use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        let l = line.unwrap();
        let re =
            Regex::new(r"target area: x=(\-?\d+)\.\.(\-?\d+), y=(\-?\d+)\.\.(\-?\d+)$").unwrap();
        let groups = re.captures(l.trim()).unwrap();
        let x0: isize = groups.get(1).unwrap().as_str().parse().unwrap();
        let x1: isize = groups.get(2).unwrap().as_str().parse().unwrap();
        let y0: isize = groups.get(3).unwrap().as_str().parse().unwrap();
        let y1: isize = groups.get(4).unwrap().as_str().parse().unwrap();
        let min = isize::min(y0, y1);
        println!("{} {} {} {}, {}", x0, x1, y0, y1, min);
        let v_start = -min - 1;
        println!("{}", v_start);
        let max = (v_start * v_start + v_start) / 2;
        println!("{}", max);
    }
}
