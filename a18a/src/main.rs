use crate::Part::N;
use crate::Part::P;
use std::panic;
use std::{
    io::{self, BufRead},
    iter::Peekable,
    ops::Add,
    str::Chars,
};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let a = lines.next().unwrap().unwrap();
    let mut s = a.chars().peekable();
    let mut p = Pair::parse(&mut s);
    for line in lines {
        let b = line.unwrap();
        let mut s = b.chars().peekable();
        let q = Pair::parse(&mut s);
        p = p + q;
        println!("{}: {}", p.to_string(), p.magnitude());
    }
}

enum Part {
    N(usize),
    P(Box<Pair>),
}

#[derive(Clone)]
struct Pair {
    l: Part,
    r: Part,
}

impl Clone for Part {
    fn clone(&self) -> Part {
        match self {
            N(usize) => N(*usize),
            P(b) => P(b.clone()),
        }
    }
}

impl Pair {
    fn to_string(&self) -> String {
        let l = match &self.l {
            N(i) => i.to_string(),
            P(b) => b.to_string(),
        };
        let r = match &self.r {
            N(i) => i.to_string(),
            P(b) => b.to_string(),
        };
        return format!("[{},{}]", l, r);
    }

    fn magnitude(&self) -> usize {
        let ml = match &self.l {
            N(l) => *l,
            P(r) => r.as_ref().magnitude(),
        };
        let mr = match &self.r {
            N(l) => *l,
            P(r) => r.as_ref().magnitude(),
        };
        return ml * 3 + mr * 2;
    }

    fn parse(s: &mut Peekable<Chars>) -> Pair {
        s.next().expect("missing [");
        let c = s.peek().unwrap();
        let l = match c {
            '[' => P(Box::new(Pair::parse(s))),
            _ => N(s.next().unwrap().to_digit(10).unwrap() as usize),
        };
        s.next().expect("missing ,");
        let c = s.peek().unwrap();
        let r = match c {
            '[' => P(Box::new(Pair::parse(s))),
            _ => N(s.next().unwrap().to_digit(10).unwrap() as usize),
        };
        s.next().expect("missing ]");
        Pair { l, r }
    }

    fn depth_max(&self) -> usize {
        let d = self.depth();
        return usize::max(d.0, d.1);
    }

    fn depth(&self) -> (usize, usize) {
        let dl = match &self.l {
            N(_) => 1,
            P(b) => b.depth_max() + 1,
        };
        let dr = match &self.r {
            N(_) => 1,
            P(b) => b.depth_max() + 1,
        };
        (dl, dr)
    }

    fn push_l(self, n: usize) -> Pair {
        if n == 0 {
            return self;
        }
        match self.l {
            N(i) => Pair {
                l: N(i + n),
                r: self.r,
            },
            P(b) => Pair {
                l: P(Box::new(b.push_l(n))),
                r: self.r,
            },
        }
    }

    fn push_r(self, n: usize) -> Pair {
        if n == 0 {
            return self;
        }
        match self.r {
            N(i) => Pair {
                l: self.l,
                r: N(i + n),
            },
            P(b) => Pair {
                l: self.l,
                r: P(Box::new(b.push_r(n))),
            },
        }
    }

    fn explode(self, depth: usize) -> (usize, Pair, usize) {
        if let P(ref b) = self.l {
            if depth == 3 {
                match &b.l {
                    N(l) => {
                        if let N(r) = b.r {
                            let x = match self.r {
                                N(my_r) => (
                                    *l,
                                    Pair {
                                        l: N(0),
                                        r: N(r + my_r),
                                    },
                                    0,
                                ),
                                P(my_r) => (
                                    *l,
                                    Pair {
                                        l: N(0),
                                        r: P(Box::new(my_r.push_l(r))),
                                    },
                                    0,
                                ),
                            };
                            return x;
                        }
                    }
                    P(x) => panic!("Explode failed {}", x.to_string()),
                }
            } else if depth + b.depth_max() > 3 {
                let (l, p, r) = b.clone().explode(depth + 1);
                return (
                    l,
                    match self.r {
                        N(n) => Pair {
                            l: P(Box::new(p)),
                            r: N(n + r),
                        },
                        P(b) => Pair {
                            l: P(Box::new(p)),
                            r: P(Box::new(b.push_l(r))),
                        },
                    },
                    0,
                );
            }
        }
        if let P(ref b) = self.r {
            if depth == 3 {
                match &b.r {
                    N(r) => {
                        if let N(l) = b.l {
                            return match self.l {
                                N(my_l) => (
                                    0,
                                    Pair {
                                        l: N(l + my_l),
                                        r: N(0),
                                    },
                                    *r,
                                ),
                                P(my_l) => (
                                    0,
                                    Pair {
                                        l: P(Box::new(my_l.push_r(l))),
                                        r: N(0),
                                    },
                                    *r,
                                ),
                            };
                        }
                    }
                    P(x) => panic!("Explode failed {}", x.to_string()),
                }
            } else if depth + b.depth_max() > 3 {
                let (l, p, r) = b.clone().explode(depth + 1);
                return (
                    0,
                    match self.l {
                        N(n) => Pair {
                            l: N(n + l),
                            r: P(Box::new(p)),
                        },
                        P(b) => Pair {
                            l: P(Box::new(b.push_r(l))),
                            r: P(Box::new(p)),
                        },
                    },
                    r,
                );
            }
        }
        (0, self, 0)
    }

    fn need_split(&self) -> bool {
        let l = match &self.l {
            N(i) => *i > 9,
            P(b) => b.need_split(),
        };
        let r = match &self.r {
            N(i) => *i > 9,
            P(b) => b.need_split(),
        };
        l || r
    }

    fn split(self) -> Pair {
        match &self.l {
            N(l) => {
                if *l > 9 {
                    return Pair {
                        l: P(Box::new(Pair {
                            l: N(l / 2),
                            r: N(l / 2 + l % 2),
                        })),
                        r: self.r,
                    };
                }
            }
            P(b) => {
                if b.need_split() {
                    return Pair {
                        l: P(Box::new(b.clone().split())),
                        r: self.r,
                    };
                }
            }
        }
        match &self.r {
            N(r) => {
                if *r > 9 {
                    return Pair {
                        l: self.l,
                        r: P(Box::new(Pair {
                            l: N(r / 2),
                            r: N(r / 2 + r % 2),
                        })),
                    };
                }
            }
            P(b) => {
                if b.need_split() {
                    return Pair {
                        l: self.l,
                        r: P(Box::new(b.clone().split())),
                    };
                }
            }
        }
        self
    }

    fn reduce(self) -> Pair {
        let mut p = self;
        println!("reduce: {}", p.to_string());
        loop {
            let d = p.depth_max();
            if d == 5 {
                p = p.explode(0).1;
                println!("after explode: {}", p.to_string());
                continue;
            } else if p.need_split() {
                p = p.split();
                println!("after split:   {}", p.to_string());
                continue;
            }
            return p;
        }
    }
}

impl Add for Pair {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            l: P(Box::new(self)),
            r: P(Box::new(other)),
        }
        .reduce()
    }
}
