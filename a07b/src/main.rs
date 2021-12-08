use std::{
    collections::BTreeMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i64>>();
    let mut map: BTreeMap<i64, i64> = BTreeMap::new();
    for i in n.iter() {
        *map.entry(*i).or_default() += 1;
    }
    let clone = map.clone();
    let max = clone.keys().max().unwrap();
    let sum = cost(&map, 0);
    let mut min = sum;
    for i in 0..*max {
        let c = cost(&map, i);
        if c < min {
            min = c;
        }
    }
    println!("{}", min);
}

fn cost(map: &BTreeMap<i64, i64>, target: i64) -> i64 {
    let mut sum = 0i64;
    for (k, v) in map.iter() {
        let diff = (*k - target).abs();
        sum += diff * (diff + 1) / 2 * *v;
    }
    return sum;
}
