use std::{
    borrow::BorrowMut,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut grid: Vec<Vec<Node>> = Vec::new();
    for line in lines {
        let l = line.unwrap();
        let row = l
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .map(|d| Node {
                cost: d,
                dist: 0,
                heur: 0,
                neighbors: Vec::new(),
            })
            .collect();
        grid.push(row);
    }
    let h = grid.len().clone();
    let w = grid[0].len().clone();
    for y in 0..h {
        for x in 0..w {
            let mut n = &grid[y][x];
            for c in [
                grid.g,
                grid[y][x + 1],
                grid[y + 1][x],
                grid[y - 1][x],
            ] {
                n.neighbors.push(c);
            }
        }
    }

    // calc heuristic+dist
    // step
    // update neighbors

    println!("0");
}

struct Node {
    cost: u32,
    dist: u32,
    heur: u32,
    neighbors: Vec<Node>,
}

impl Node {
    fn cc(&self) -> u32 {
        return self.dist + self.heur;
    }
}
