use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    io::{self, BufRead, Error},
};

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut scanners: Vec<Scanner> = Vec::new();
    'scan: loop {
        lines.next();
        let mut beacons: Vec<[isize; 3]> = Vec::new();
        loop {
            let pos = lines.next();
            match pos {
                None => {
                    scanners.push(Scanner {
                        beacons,
                        orientation: Cell::new(0),
                        position: RefCell::new([0, 0, 0]),
                        aligned: Cell::new(false),
                    });
                    break 'scan;
                }
                Some(l) => {
                    let l = l.unwrap();
                    if l.is_empty() {
                        scanners.push(Scanner {
                            beacons,
                            orientation: Cell::new(0),
                            position: RefCell::new([0, 0, 0]),
                            aligned: Cell::new(false),
                        });
                        break;
                    }
                    beacons.push(
                        l.split(",")
                            .map(|p| p.parse::<isize>().unwrap())
                            .collect::<Vec<isize>>()
                            .as_slice()
                            .try_into()
                            .unwrap(),
                    );
                }
            }
        }
    }
    let mut beacons: HashSet<[isize; 3]> = HashSet::from_iter(scanners[0].beacons.iter().cloned());
    scanners[0].aligned.set(true);
    while !scanners.iter().all(|s| s.aligned.get()) {
        println!(
            "{}/{} scanners aligned",
            scanners.iter().filter(|s| s.aligned.get()).count(),
            scanners.len()
        );
        for s_origin in scanners.iter().filter(|s| s.aligned.get()) {
            'scanner: for s in scanners.iter().filter(|s| !s.aligned.get()) {
                let mut matches: HashSet<usize> = HashSet::new();
                for (id, origin) in s_origin.distances().iter().enumerate() {
                    for target in s.distances().iter_mut() {
                        let mut count = 0;
                        for (k, v) in origin {
                            let q = target.entry(*k).or_default();
                            count += usize::min(*q, *v);
                        }
                        if count > 10 {
                            matches.insert(id);
                        }
                    }
                }
                if matches.len() >= 12 {
                    for axis in 0..AXIS.len() {
                        s.orientation.set(axis);
                        let mut m = matches.iter();
                        let o = m.next().unwrap();
                        let mut count = 0;
                        while let Some(p) = m.next() {
                            let bo = &s_origin.normalized_beacons()[*o]; // origin beacon
                            let br = &s_origin.normalized_beacons()[*p]; // reference beacon
                            let bv = [br[0] - bo[0], br[1] - bo[1], br[2] - bo[2]]; // vector from origin to reference beacon
                            for to in s.normalized_beacons().iter() {
                                for tr in s.normalized_beacons().iter() {
                                    let tv = [tr[0] - to[0], tr[1] - to[1], tr[2] - to[2]]; // target vector
                                    if bv[0] == tv[0] && bv[1] == tv[1] && bv[2] == tv[2] {
                                        count += 1;
                                        if count == 11 {
                                            *s.position.borrow_mut() =
                                                [bo[0] - to[0], bo[1] - to[1], bo[2] - to[2]];
                                            s.aligned.set(true);
                                            break 'scanner;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    panic!("no axis match found",);
                }
            }
        }
    }
    for b in scanners.iter().flat_map(|s| s.normalized_beacons()) {
        beacons.insert(b.clone());
    }
    println!("beacons total {}", beacons.len());
    let mut md = 0;
    for sa in scanners.iter() {
        for sb in scanners.iter() {
            let a = sa.position.borrow();
            let b = sb.position.borrow();
            let dist = (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs();
            md = isize::max(md, dist);
        }
    }
    println!("max dist {}", md);
    Ok(())
}

const AXIS: [[[isize; 3]; 3]; 24] = [
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
    [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
    [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
    [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
    [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
    [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
    [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
    [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
    [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
    [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
];

struct Scanner {
    beacons: Vec<[isize; 3]>,
    orientation: Cell<usize>,
    position: RefCell<[isize; 3]>,
    aligned: Cell<bool>,
}

impl Scanner {
    fn normalized_beacons(&self) -> Vec<[isize; 3]> {
        self.beacons
            .iter()
            .map(|b| {
                let axis = AXIS[self.orientation.get()];
                let pos = self.position.borrow();
                let x = b[0] * axis[0][0] + b[1] * axis[0][1] + b[2] * axis[0][2] + pos[0];
                let y = b[0] * axis[1][0] + b[1] * axis[1][1] + b[2] * axis[1][2] + pos[1];
                let z = b[0] * axis[2][0] + b[1] * axis[2][1] + b[2] * axis[2][2] + pos[2];
                [x, y, z]
            })
            .collect()
    }

    fn distances(&self) -> Vec<HashMap<isize, usize>> {
        self.beacons
            .iter()
            .map(|b| {
                let mut qubic_dists = HashMap::new();
                for c in self.beacons.iter() {
                    if b == c {
                        continue;
                    }
                    let d = (c[0] - b[0]) * (c[0] - b[0])
                        + (c[1] - b[1]) * (c[1] - b[1])
                        + (c[2] - b[2]) * (c[2] - b[2]);
                    *qubic_dists.entry(d).or_default() += 1;
                }
                qubic_dists
            })
            .collect::<Vec<HashMap<isize, usize>>>()
    }
}
