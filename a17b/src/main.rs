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
        println!("x:{}..{} y:{}..{}", x0, x1, y0, y1);
        let v_start = -min - 1;
        let y_max = (v_start * v_start + v_start) / 2;
        println!("v_start: {} -> y_max: {}", v_start, y_max);
        let mut count = 0;
        for x in 1..x1 + 1 {
            for y in y0..v_start + 1 {
                let mut sx = x;
                let mut sy = y;
                let mut px = 0;
                let mut py = 0;
                loop {
                    px += sx;
                    py += sy;
                    if px >= x0 && px <= x1 && py >= y0 && py <= y1 {
                        count += 1;
                        break;
                    }
                    if sx > 0 {
                        sx -= 1;
                    }
                    sy -= 1;
                    if px > x1 || py < y0 {
                        break;
                    }
                }
            }
        }
        println!("{}", count);
    }
}
