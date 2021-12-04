use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let numbers = lines.next().unwrap().unwrap();
    lines.next();
    println!("{}", numbers);
    let numbers: Vec<i64> = numbers.split(",").map(|n| n.parse().unwrap()).collect();
    let mut fields: Vec<Field> = Vec::new();
    let mut f = Field::new();
    while let Some(line) = lines.next() {
        let l = line.unwrap();
        println!("{} ({})", l, f.field.len());
        if l.trim().is_empty() {
            continue;
        }
        let line_nums: Vec<i64> = l
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        f.add_line(line_nums);
        if f.field.len() == 5 {
            fields.push(f);
            f = Field::new();
        }
    }
    for n in numbers {
        for (i, f) in fields.iter_mut().enumerate() {
            f.drop(n);
            if f.is_win() {
                let s = f.sum();
                println!("{} wins: {} * {} = {}", i, s, n, s * n); //unmarked * drawn);
                return;
            }
        }
    }
    println!("{}", fields.len());
}

struct Field {
    field: Vec<[Option<i64>; 5]>,
}

impl Field {
    pub fn new() -> Field {
        return Field { field: Vec::new() };
    }

    pub fn add_line(&mut self, line: Vec<i64>) {
        let mut a = [None; 5];
        for (i, n) in line.iter().enumerate() {
            a[i] = Some(n.clone());
        }
        self.field.push(a);
    }

    pub fn drop(&mut self, n: i64) {
        for line in self.field.iter_mut() {
            *line = line.map(|i| if i == Some(n) { None } else { i });
        }
    }

    pub fn sum(&self) -> i64 {
        let mut sum = 0;
        for l in self.field.iter() {
            for n in l.iter() {
                sum += n.unwrap_or_default();
            }
        }
        return sum;
    }
    pub fn is_win(&self) -> bool {
        for line in self.field.iter() {
            if line.iter().all(|n| n == &None) {
                return true;
            }
        }
        for i in 0..5 {
            if self
                .field
                .iter()
                .map(|n| n.get(i).unwrap())
                .all(|n| n == &None)
            {
                return true;
            }
        }
        return false;
    }
}
