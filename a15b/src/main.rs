use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut grid: HashMap<(isize, isize), RefCell<Node>> = HashMap::new();
    let mut h = 0;
    let mut w = 0;
    for line in lines {
        w = 0;
        let l = line.unwrap();
        let row = l.chars().map(|c| c.to_digit(10).unwrap());
        for c in row {
            let n = RefCell::new(Node {
                x: w,
                y: h,
                cost: c as isize,
                dist: None,
                closed: false,
            });
            grid.insert((w, h), n);
            w += 1;
        }
        h += 1;
    }
    let mut monster_grid: HashMap<(isize, isize), RefCell<Node>> = HashMap::new();
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
                            x,
                            y,
                            cost: (p.borrow().cost + yi + xi - 1) % 9 + 1,
                            dist: None,
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

    let start = grid.get(&(0, 0)).unwrap();
    {
        let mut start = start.borrow_mut();
        start.dist = Some(0);
    }
    let mut buckets: BTreeMap<isize, Vec<&RefCell<Node>>> = BTreeMap::new();
    buckets.insert(0, vec![start; 1]);
    loop {
        let best = buckets
            .iter_mut()
            .filter(|b| !b.1.is_empty())
            .next()
            .unwrap()
            .1
            .pop();
        buckets.retain(|_, v| !v.is_empty());
        {
            let mut p = best.unwrap().borrow_mut();
            p.closed = true;
        }
        let p = best.unwrap().borrow();
        let mut next: Vec<&RefCell<Node>> = Vec::new();
        for adj_best in [
            (p.x, p.y - 1),
            (p.x, p.y + 1),
            (p.x + 1, p.y),
            (p.x - 1, p.y),
        ]
        .iter()
        {
            if let Some(exists) = grid.get(adj_best) {
                next.push(exists);
            }
        }
        for c in next.iter() {
            {
                let mut next = c.borrow_mut();
                if next.closed {
                    continue;
                }
                let dist = p.dist.unwrap();
                let d = match next.dist {
                    Some(d) => {
                        let q = isize::min(d, dist + next.cost);
                        buckets
                            .entry(d)
                            .and_modify(|e| e.retain(|df| df.try_borrow().is_ok()));
                        q
                    }
                    None => dist + next.cost,
                };

                next.dist = Some(d);
                buckets.entry(d).or_insert(Vec::new()).push(c);

                if next.x == w - 1 && next.y == h - 1 {
                    println!("{}", next.dist.unwrap());
                    return;
                }
            }
        }
    }
}

struct Node {
    x: isize,
    y: isize,
    dist: Option<isize>,
    cost: isize,
    closed: bool,
}
