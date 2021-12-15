use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut grid: HashMap<(i32, i32), RefCell<Node>> = HashMap::new();
    let mut h = 0;
    let mut w = 0;
    for line in lines {
        w = 0;
        let l = line.unwrap();
        let row = l.chars().map(|c| c.to_digit(10).unwrap());
        for c in row {
            let n = RefCell::new(Node {
                cost: c as i32,
                dist: None,
                heur: 1,
                closed: false,
            });
            grid.insert((w, h), n);
            w += 1;
        }
        h += 1;
    }
    let mut monster_grid: HashMap<(i32, i32), RefCell<Node>> = HashMap::new();
    for yi in 0..5 {
        for xi in 0..5 {
            for py in 0..h {
                for px in 0..w {
                    let p = grid.get(&(px, py)).unwrap();
                    let x = px + xi * w;
                    let y = py + yi * h;
                    monster_grid.insert(
                        (x, y),
                        RefCell::new(Node {
                            cost: (p.borrow().cost + yi + xi - 1) % 9 + 1,
                            dist: None,
                            heur: (w * 5 - x + h * 5 - y) * 1,
                            closed: false,
                        }),
                    );
                }
            }
        }
    }
    w *= 5;
    h *= 5;
    grid = monster_grid;
    // for y in 0..h {
    //     for x in 0..w {
    //         print!("{}", grid.get(&(x, y)).unwrap().borrow().cost);
    //     }
    //     println!();
    // }

    let start = grid.get(&(0, 0)).unwrap();
    {
        let mut start = start.borrow_mut();
        start.dist = Some(0);
    }
    let mut current: HashMap<(i32, i32), &RefCell<Node>> = HashMap::new();
    current.insert((0, 0), start);
    loop {
        let (p, best) = current
            .iter()
            .filter(|&(_, c)| !c.borrow().closed && c.borrow().dist != None)
            .min_by_key(|&(_, c)| c.borrow().dist.unwrap() + c.borrow().heur)
            .unwrap();
        best.borrow_mut().closed = true;
        let mut next: HashMap<(i32, i32), &RefCell<Node>> = HashMap::new();
        for adj_best in [
            (p.0, p.1 - 1),
            (p.0, p.1 + 1),
            (p.0 + 1, p.1),
            (p.0 - 1, p.1),
        ]
        .iter()
        {
            if let Some(exists) = grid.get(adj_best) {
                next.insert(*adj_best, exists);
            }
        }
        for (p, c) in next.iter() {
            {
                let mut e = c.borrow_mut();
                for np in [
                    (p.0, p.1 - 1),
                    (p.0, p.1 + 1),
                    (p.0 + 1, p.1),
                    (p.0 - 1, p.1),
                ]
                .iter()
                {
                    if let Some(back) = grid.get(np) {
                        let b = back.borrow();
                        if let Some(dist) = b.dist {
                            e.dist = Some(match e.dist {
                                Some(d) => i32::min(d, dist + e.cost),
                                None => dist + e.cost,
                            })
                        }
                    }
                }
                if p.0 == w - 1 && p.1 == h - 1 {
                    println!("{}", e.dist.unwrap());
                    return;
                }
            }
        }
        current.extend(next.into_iter());
    }
}

struct Node {
    dist: Option<i32>,
    cost: i32,
    heur: i32,
    closed: bool,
}
