use crate::Inst::{Add, Div, Eql, Inp, Mod, Mul};
use crate::Var::{Lit, W, X, Y, Z};
use std::io::{self, BufRead};
use std::panic;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let prog: Vec<Inst> = lines
        .map(|l| {
            let l = l.unwrap();
            let mut s = l.split_whitespace();
            let i = s.next();
            let params: Vec<Var> = s
                .map(|p| match p {
                    "w" => W,
                    "x" => X,
                    "y" => Y,
                    "z" => Z,
                    n @ _ => Lit(n.parse().unwrap()),
                })
                .collect();
            let i = match i.unwrap() {
                "inp" => Inp(params[0]),
                "add" => Add(params[0], params[1]),
                "mul" => Mul(params[0], params[1]),
                "div" => Div(params[0], params[1]),
                "mod" => Mod(params[0], params[1]),
                "eql" => Eql(params[0], params[1]),
                _ => panic!(),
            };
            i
        })
        .collect();
    let mut pa = vec![];
    let mut pb = vec![];
    let mut pc = vec![];
    for i in 0..14 {
        match prog[18 * i + 4] {
            Div(_, Lit(p)) => pa.push(p),
            q => panic!("{:?}", q),
        };
        match prog[18 * i + 5] {
            Add(_, Lit(p)) => pb.push(p),
            _ => panic!(),
        };
        match prog[18 * i + 15] {
            Add(_, Lit(p)) => pc.push(p),
            _ => panic!(),
        };
    }
    let mut input = [1; 14];
    let mut stack: Vec<(usize, i64)> = vec![];
    for (i, a) in pa.iter().enumerate() {
        if *a == 1 {
            stack.push((i, pc[i]));
        } else {
            let (r, z) = stack.pop().unwrap();
            let diff = pb[i] + z;
            if diff > 0 {
                input[i] += diff;
            } else {
                input[r] -= diff;
            }
        }
    }
    print!("code: ");
    for i in input {
        print!("{}", i);
    }
    println!();
    let mut vars = [0; 4];
    let mut i = 0;
    for cmd in prog.iter() {
        match cmd {
            Inp(a) => {
                vars[a.u()] = input[i];
                i += 1;
            }
            Add(a, b) => vars[a.u()] = a.i(vars) + b.i(vars),
            Mul(a, b) => vars[a.u()] = a.i(vars) * b.i(vars),
            Div(a, b) => vars[a.u()] = a.i(vars) / b.i(vars),
            Mod(a, b) => vars[a.u()] = a.i(vars) % b.i(vars),
            Eql(a, b) => vars[a.u()] = if a.i(vars) == b.i(vars) { 1 } else { 0 },
        }
    }
    println!("result: {}", vars[3]);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Var {
    W,
    X,
    Y,
    Z,
    Lit(i64),
}

impl Var {
    fn u(&self) -> usize {
        match self {
            W => 0,
            X => 1,
            Y => 2,
            Z => 3,
            _ => panic!(),
        }
    }

    fn i(&self, vars: [i64; 4]) -> i64 {
        match self {
            W => vars[0],
            X => vars[1],
            Y => vars[2],
            Z => vars[3],
            Lit(i) => *i,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]

enum Inst {
    Inp(Var),
    Add(Var, Var),
    Mul(Var, Var),
    Div(Var, Var),
    Mod(Var, Var),
    Eql(Var, Var),
}
