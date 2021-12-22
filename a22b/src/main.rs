use std::{
    collections::BTreeSet,
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
            edges: [
                Edge { from: x0, to: x1 },
                Edge { from: y0, to: y1 },
                Edge { from: z0, to: z1 },
            ]
            .to_vec(),
        };
        println!("{}", c);
        cubes.push(c);
    }
    let mut space: Vec<BTreeSet<isize>> = vec![BTreeSet::new(); 3];
    for c in cubes.iter() {
        for d in 0..3 {
            space[d].insert(c.edges[d].from);
            space[d].insert(c.edges[d].to + 1);
        }
    }
    let space: Vec<Vec<&isize>> = Vec::from_iter(space.iter().map(|s| Vec::from_iter(s.iter())));
    println!(
        "{} cubes: {} {} {}",
        cubes.len(),
        space[0].len(),
        space[1].len(),
        space[2].len()
    );
    let mut reactor: Vec<Vec<Vec<bool>>> =
        vec![vec![vec![false; space[2].len()]; space[1].len()]; space[0].len()];
    let index_cubes: Vec<Cube> = cubes
        .iter()
        .map(|c| Cube {
            on: c.on,
            edges: c
                .edges
                .iter()
                .enumerate()
                .map(|(i, e)| Edge {
                    from: space[i].iter().position(|p| **p == e.from).unwrap() as isize,
                    to: space[i].iter().position(|p| **p == e.to + 1).unwrap() as isize,
                })
                .collect(),
        })
        .collect();
    for c in index_cubes {
        let cx = &c.edges[0];
        let cy = &c.edges[1];
        let cz = &c.edges[2];
        for x in cx.from..cx.to {
            for y in cy.from..cy.to {
                for z in cz.from..cz.to {
                    reactor[x as usize][y as usize][z as usize] = c.on;
                }
            }
        }
    }
    let mut count = 0;
    for x in 0..space[0].len() - 1 {
        for y in 0..space[1].len() - 1 {
            for z in 0..space[2].len() - 1 {
                if reactor[x][y][z] {
                    count += (space[0][x + 1] - space[0][x])
                        * (space[1][y + 1] - space[1][y])
                        * (space[2][z + 1] - space[2][z]);
                }
            }
        }
    }
    println!("{}", count);
}

#[derive(Clone, Copy)]
struct Edge {
    from: isize,
    to: isize,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.from, self.to,)
    }
}

struct Cube {
    on: bool,
    edges: Vec<Edge>,
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.on, self.edges[0], self.edges[1], self.edges[2],
        )
    }
}
