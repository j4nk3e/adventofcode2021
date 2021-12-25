use std::io::{self, BufRead};

use rustc_hash::FxHashSet;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut w = 0;
    let mut e: FxHashSet<usize> = FxHashSet::default();
    let mut s: FxHashSet<usize> = FxHashSet::default();
    let mut i = 0;
    for line in lines {
        let l = line.unwrap();
        w = l.len();
        l.chars().for_each(|c| {
            match c {
                '>' => e.insert(i),
                'v' => s.insert(i),
                _ => true,
            };
            i += 1;
        });
    }
    let h = i / w;
    let mut i = 0;
    loop {
        i += 1;
        let mut fin = true;
        e = e
            .iter()
            .map(|p| {
                let x = p % w;
                let y = p / w;
                let t = (x + 1) % w + y * w;
                if e.contains(&t) || s.contains(&t) {
                    *p
                } else {
                    fin = false;
                    t
                }
            })
            .collect();
        s = s
            .iter()
            .map(|p| {
                let x = p % w;
                let y = p / w;
                let t = x + ((y + 1) % h) * w;
                if e.contains(&t) || s.contains(&t) {
                    *p
                } else {
                    fin = false;
                    t
                }
            })
            .collect();
        if fin {
            println!("{}", i);
            break;
        }
    }
}
