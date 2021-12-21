use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut max_x = 0;
    let mut max_y = 0;
    let algo: HashSet<usize> = lines
        .next()
        .unwrap()?
        .chars()
        .enumerate()
        .filter_map(|(p, b)| if b == '#' { Some(p) } else { None })
        .collect();
    lines.next();
    let mut image: HashMap<(isize, isize), bool> = HashMap::new();
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
        let mut next: HashMap<(isize, isize), bool> = HashMap::new();
        for y in -i * 2..max_y + i * 2 {
            for x in -i * 2..max_x + i * 2 {
                let mut q = 0;
                let mut shift = 0;
                for dy in -1..2 {
                    for dx in -1..2 {
                        if *image.get(&(x + dx, y + dy)).unwrap_or(&background) {
                            q |= 1 << shift;
                        };
                        shift += 1;
                    }
                }
                next.insert((x, y), algo.contains(&q));
            }
        }
        image = next;
        if !background && algo.contains(&0) {
            background = true;
        } else if background && !algo.contains(&0b111111111) {
            background = false;
        }
        image.retain(|_, v| background != *v);

        // for y in -i * 2..max_y + i * 2 {
        //     for x in -i * 2..max_x + i * 2 {
        //         print!(
        //             "{}",
        //             if *image.get(&(x, y)).unwrap_or_else(|| &background) {
        //                 '#'
        //             } else {
        //                 '.'
        //             }
        //         );
        //     }
        //     println!();
        // }
        println!(
            "{}: {}",
            i,
            image.iter().filter(|(_, v)| **v != background).count()
        );
    }
    Ok(())
}
