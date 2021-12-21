use std::{
    error::Error,
    io::{self, BufRead},
};

use rustc_hash::FxHashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut max_x = 0;
    let mut max_y = 0;
    let algo: Vec<bool> = lines.next().unwrap()?.chars().map(|c| c == '#').collect();
    lines.next();
    let mut image: FxHashMap<(isize, isize), bool> = FxHashMap::default();
    for (y, l) in lines.enumerate() {
        for (x, c) in l?.chars().map(|c| c == '#').enumerate() {
            if c {
                max_x = isize::max(max_x, x as isize);
                max_y = isize::max(max_y, y as isize);
                image.insert((x as isize, y as isize), true);
            }
        }
    }
    let mut background = false;
    for i in 1..51 {
        let mut next: FxHashMap<(isize, isize), bool> = FxHashMap::default();
        for y in -i * 2..max_y + i * 2 {
            for x in -i * 2..max_x + i * 2 {
                let mut q = 0;
                let mut shift = 0;
                for dy in -1..2 {
                    for dx in -1..2 {
                        if *image.get(&(x - dx, y - dy)).unwrap_or(&background) {
                            q |= 1 << shift;
                        };
                        shift += 1;
                    }
                }
                next.insert((x, y), algo[q]);
            }
        }
        if !background && algo[0] {
            background = true;
        } else if background && !algo[0b111111111] {
            background = false;
        }
        next.retain(|_, v| background != *v);
        image = next;
    }
    println!("{}", image.len());
    Ok(())
}
