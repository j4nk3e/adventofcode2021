use crate::Pair::N;
use crate::Pair::P;
use std::{
    io::{self, BufRead},
    iter::Peekable,
    ops::Add,
    str::Chars,
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let pairs: Vec<Pair> = lines
        .map(|l| {
            let b = l.unwrap();
            let mut s = b.chars().peekable();
            Pair::parse(&mut s)
        })
        .collect();
    let mut max = 0;
    for a in 0..pairs.len() {
        for b in 0..pairs.len() {
            if a == b {
                continue;
            }
            max = usize::max((pairs[a].clone() + pairs[b].clone()).magnitude(), max);
            max = usize::max((pairs[b].clone() + pairs[a].clone()).magnitude(), max);
        }
    }
    println!("{}", max);
}

#[derive(Clone)]
enum Pair {
    N(usize),
    P { l: Box<Pair>, r: Box<Pair> },
}

impl Pair {
    fn magnitude(&self) -> usize {
        match self {
            N(i) => *i,
            P { l, r } => l.magnitude() * 3 + r.magnitude() * 2,
        }
    }

    fn parse(s: &mut Peekable<Chars>) -> Pair {
        s.next().expect("missing [");
        let c = s.peek().unwrap();
        let l = match c {
            '[' => Pair::parse(s),
            _ => N(s.next().unwrap().to_digit(10).unwrap() as usize),
        };
        s.next().expect("missing ,");
        let c = s.peek().unwrap();
        let r = match c {
            '[' => Pair::parse(s),
            _ => N(s.next().unwrap().to_digit(10).unwrap() as usize),
        };
        s.next().expect("missing ]");
        P {
            l: Box::new(l),
            r: Box::new(r),
        }
    }

    fn depth(&self) -> usize {
        match self {
            N(_) => 0,
            P { l, r } => usize::max(l.depth(), r.depth()) + 1,
        }
    }

    fn push_l(self, n: usize) -> Pair {
        if n == 0 {
            return self;
        }
        match self {
            N(i) => N(i + n),
            P { l, r } => P {
                l: Box::new(l.push_l(n)),
                r,
            },
        }
    }

    fn push_r(self, n: usize) -> Pair {
        if n == 0 {
            return self;
        }
        match self {
            N(i) => N(i + n),
            P { l, r } => P {
                l,
                r: Box::new(r.push_r(n)),
            },
        }
    }

    fn explode(self, depth: usize) -> (usize, Pair, usize) {
        match self {
            N(i) => (0, N(i), 0),
            P { l, r } => {
                if depth == 4 {
                    (l.magnitude(), N(0), r.magnitude())
                } else if depth + l.depth() > 3 {
                    let (pl, p, pr) = l.explode(depth + 1);
                    (
                        pl,
                        P {
                            l: Box::new(p),
                            r: Box::new(r.push_l(pr)),
                        },
                        0,
                    )
                } else if depth + r.depth() > 3 {
                    let (pl, p, pr) = r.explode(depth + 1);
                    (
                        0,
                        P {
                            l: Box::new(l.push_r(pl)),
                            r: Box::new(p),
                        },
                        pr,
                    )
                } else {
                    (0, P { l, r }, 0)
                }
            }
        }
    }

    fn need_split(&self) -> bool {
        match self {
            N(i) => *i > 9,
            P { l, r } => l.need_split() || r.need_split(),
        }
    }

    fn split(self) -> Pair {
        match self {
            N(i) => {
                if i > 9 {
                    P {
                        l: Box::new(N(i / 2)),
                        r: Box::new(N(i / 2 + i % 2)),
                    }
                } else {
                    N(i)
                }
            }
            P { l, r } => {
                if l.need_split() {
                    P {
                        l: Box::new(l.split()),
                        r,
                    }
                } else if r.need_split() {
                    P {
                        l,
                        r: Box::new(r.split()),
                    }
                } else {
                    P { l, r }
                }
            }
        }
    }

    fn reduce(self) -> Pair {
        let mut p = self;
        loop {
            let d = p.depth();
            if d == 5 {
                p = p.explode(0).1;
                continue;
            } else if p.need_split() {
                p = p.split();
                continue;
            }
            return p;
        }
    }
}

impl Add for Pair {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        P {
            l: Box::new(self),
            r: Box::new(other),
        }
        .reduce()
    }
}
