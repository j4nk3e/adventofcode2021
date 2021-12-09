use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut field: HashMap<(isize, isize), u8> = HashMap::new();
    let mut y = 0isize;
    for line in lines {
        let line = line.unwrap();
        let mut x = 0isize;
        for c in line.chars() {
            let h = c as u8 - b'0';
            if h < 9 {
                field.insert((y, x), h);
            }
            x += 1;
        }
        y += 1;
    }
    let mut basins: Vec<usize> = Vec::new();
    while !field.is_empty() {
        let k = field.keys().next().unwrap().clone();
        let f = field.remove(&k).unwrap();
        if f == 9 {
            continue;
        }
        let res = sum_adjacent(&mut field, k);
        basins.push(res);
    }
    basins.sort();
    basins.reverse();
    let mut i = basins.iter();
    let c = i.next().unwrap() * i.next().unwrap() * i.next().unwrap();
    println!("{}", c);
}

fn sum_adjacent(field: &mut HashMap<(isize, isize), u8>, pos: (isize, isize)) -> usize {
    let mut sum = 1;
    let mut adj = Vec::new();
    for z in [
        (pos.0, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
    ] {
        let q = field.remove(&z);
        match q {
            Some(9) => (),
            None => (),
            _ => adj.push(z),
        }
    }
    let f = field;
    for p in adj {
        sum += sum_adjacent(f, p);
    }
    return sum;
}
