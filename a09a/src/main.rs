use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut field: Vec<Vec<u8>> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            row.push(c as u8 - b'0');
        }
        field.push(row);
    }
    let mut count = 0u64;
    for y in 0..field.len() {
        let row = &field[y];
        for x in 0..row.len() {
            let i = row[x];
            let adj = get_adjacent(&field, y, x);
            if adj.iter().all(|q| q > &i) {
                count += i as u64 + 1;
            }
        }
    }
    println!("{}", &count);
}

fn get_adjacent(field: &Vec<Vec<u8>>, y: usize, x: usize) -> Vec<u8> {
    let mut adjacent = Vec::new();
    let row = &field[y];
    if x > 0 {
        adjacent.push(row[x - 1]);
    }
    if x < row.len() - 1 {
        adjacent.push(row[x + 1]);
    }
    if y > 0 {
        adjacent.push(field[y - 1][x]);
    }
    if y < field.len() - 1 {
        adjacent.push(field[y + 1][x]);
    }
    return adjacent;
}
