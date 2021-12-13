use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut dots: HashSet<(u32, u32)> = HashSet::new();
    let mut folds = false;
    for line in lines {
        let l = line.unwrap();
        if l.is_empty() {
            folds = true;
            continue;
        }
        if !folds {
            let coords: Vec<u32> = l.split(",").map(|n| n.parse().unwrap()).collect();
            dots.insert((coords[0], coords[1]));
        } else {
            let fold: Vec<&str> = l.split_whitespace().last().unwrap().split("=").collect();
            let axis = fold[0];
            let pos = fold[1].parse().unwrap();
            dots = dots
                .iter()
                .map(|(x, y)| {
                    if axis == "x" {
                        if *x > pos {
                            return (2 * pos - x, *y);
                        }
                    } else {
                        if *y > pos {
                            return (*x, 2 * pos - y);
                        }
                    }
                    return (*x, *y);
                })
                .collect();
            break;
        }
    }

    let count = dots.len();
    println!("{}", count);
}
