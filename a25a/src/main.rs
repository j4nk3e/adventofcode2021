use std::io::{self, BufRead};

use bitvec::{bitvec, order::Lsb0, prelude::BitVec};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut w = 0;
    let mut e: BitVec<Lsb0, usize> = bitvec![];
    let mut s: BitVec<Lsb0, usize> = bitvec![];
    for line in lines {
        let l = line.unwrap();
        w = l.len();
        l.chars().for_each(|c| {
            let (eb, sb) = match c {
                '>' => (true, false),
                'v' => (false, true),
                _ => (false, false),
            };
            e.push(eb);
            s.push(sb);
        });
    }
    let mut i = 0;
    loop {
        i += 1;
        let mut m = 0;
        let mut n_e: BitVec<Lsb0, usize> = BitVec::with_capacity(e.len());
        let mut n_s: BitVec<Lsb0, usize> = BitVec::with_capacity(s.len());
        n_e.resize(e.len(), false);
        n_s.resize(s.len(), false);
        for p in e.iter_ones() {
            let x = p % w;
            let y = p / w;
            let t = (x + 1) % w + y * w;
            n_e.set(
                if e[t] || s[t] {
                    p
                } else {
                    m += 1;
                    t
                },
                true,
            );
        }
        for p in s.iter_ones() {
            let x = p % w;
            let y = p / w;
            let t = x + ((y + 1) % (s.len() / w)) * w;
            n_s.set(
                if n_e[t] || s[t] {
                    p
                } else {
                    m += 1;
                    t
                },
                true,
            );
        }
        if m == 0 {
            println!("{}", i);
            break;
        }
        e = n_e;
        s = n_s;
    }
}
