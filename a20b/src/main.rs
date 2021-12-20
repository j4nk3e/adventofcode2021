use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let algo: HashSet<usize> = lines
        .next()
        .unwrap()?
        .chars()
        .enumerate()
        .filter_map(|(p, b)| if b == '#' { Some(p) } else { None })
        .collect();
    lines.next();
    let mut image: HashSet<(isize, isize)> = HashSet::new();
    for (y, l) in lines.enumerate() {
        for (x, c) in l?.chars().map(|c| c == '#').enumerate() {
            if c {
                image.insert((x as isize, y as isize));
            }
        }
    }
    let mut inverted = false;
    for _ in 0..2 {
        let mut next: HashMap<(isize, isize), usize> = HashMap::new();
        for (x, y) in image.iter() {
            if algo.contains(&0) {
                inverted = !inverted;
            }
            *next.entry((*x - 1, *y - 1)).or_default() |= 1 << 0;
            *next.entry((*x, *y - 1)).or_default() |= 1 << 1;
            *next.entry((*x + 1, *y - 1)).or_default() |= 1 << 2;
            *next.entry((*x - 1, *y)).or_default() |= 1 << 3;
            *next.entry((*x, *y)).or_default() |= 1 << 4;
            *next.entry((*x + 1, *y)).or_default() |= 1 << 5;
            *next.entry((*x - 1, *y + 1)).or_default() |= 1 << 6;
            *next.entry((*x, *y + 1)).or_default() |= 1 << 7;
            *next.entry((*x + 1, *y + 1)).or_default() |= 1 << 8;
        }
        image.clear();
        for (p, v) in next.iter() {
            if algo.contains(v) {
                image.insert(*p);
            }
        }
        for y in -5..105isize {
            for x in -5..105isize {
                print!("{}", if image.contains(&(x, y)) { '#' } else { '.' });
            }
            println!();
        }
        println!("{}", image.len());
    }
    Ok(())
}
