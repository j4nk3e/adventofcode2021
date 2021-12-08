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
    let count: i64 = n.len() as i64;
    let mut left = 0i64;
    let mut mid = 0i64;
    for (k, v) in map.iter() {
        left += v;
        if left > (count / 2) as i64 {
            mid = *k;
            break;
        }
    }
    let mut sum = 0i64;
    for (k, v) in map.iter() {
        sum += (*k - mid).abs() * *v;
    }
    println!("{}", sum);
}
