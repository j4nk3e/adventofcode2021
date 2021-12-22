use std::{
    fmt,
    io::{self, BufRead},
};

use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let re =
        Regex::new(r"^(o[nf]+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$").unwrap();
    let mut cubes: Vec<Cube> = Vec::new();
    for line in lines {
        let l = line.unwrap();
        let m = re.captures_iter(l.as_str()).next().unwrap();
        let on = m.get(1).unwrap().as_str() == "on";
        let x0: isize = m.get(2).unwrap().as_str().parse().unwrap();
        let x1: isize = m.get(3).unwrap().as_str().parse().unwrap();
        let y0: isize = m.get(4).unwrap().as_str().parse().unwrap();
        let y1: isize = m.get(5).unwrap().as_str().parse().unwrap();
        let z0: isize = m.get(6).unwrap().as_str().parse().unwrap();
        let z1: isize = m.get(7).unwrap().as_str().parse().unwrap();
        let c = Cube {
            on,
            x0,
            x1,
            y0,
            y1,
            z0,
            z1,
        };
        println!("{}", c);
        cubes.push(c);
    }
    cubes.reverse();
    let mut count = 0;
    for z in -50..=50 {
        for y in -50..=50 {
            for x in -50..=50 {
                count += match cubes.iter().find(|c| c.contains(x, y, z)) {
                    Some(c) if c.on => 1,
                    _ => 0,
                };
            }
        }
    }
    println!("{}", count);
}

struct Cube {
    on: bool,
    x0: isize,
    x1: isize,
    y0: isize,
    y1: isize,
    z0: isize,
    z1: isize,
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}..{} {}..{} {}..{}",
            self.on, self.x0, self.x1, self.y0, self.y1, self.z0, self.z1,
        )
    }
}

impl Cube {
    fn contains(&self, x: isize, y: isize, z: isize) -> bool {
        x >= self.x0 && x <= self.x1 && y >= self.y0 && y <= self.y1 && z >= self.z0 && z <= self.z1
    }
}
