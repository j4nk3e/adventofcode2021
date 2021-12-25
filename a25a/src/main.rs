use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut w = 0;
    let mut h = 0;
    let mut e: HashSet<(usize, usize)> = HashSet::new();
    let mut s: HashSet<(usize, usize)> = HashSet::new();
    for line in lines {
        let l = line.unwrap();
        w = l.len();
        l.chars().enumerate().for_each(|(x, c)| {
            match c {
                '>' => e.insert((x, h)),
                'v' => s.insert((x, h)),
                _ => true,
            };
        });
        h += 1;
    }
    let mut i = 0;
    loop {
        i += 1;
        let mut m = 0;
        let mut n_e: HashSet<(usize, usize)> = HashSet::new();
        for (x, y) in e.iter() {
            let t = ((x + 1) % w, *y);
            n_e.insert(if e.contains(&t) || s.contains(&t) {
                (*x, *y)
            } else {
                m += 1;
                t
            });
        }
        let mut n_s: HashSet<(usize, usize)> = HashSet::new();
        for (x, y) in s.iter() {
            let t = (*x, (*y + 1) % h);
            n_s.insert(if n_e.contains(&t) || s.contains(&t) {
                (*x, *y)
            } else {
                m += 1;
                t
            });
        }
        if m == 0 {
            println!("{}", i);
            break;
        }
        e = n_e;
        s = n_s;
    }
}
