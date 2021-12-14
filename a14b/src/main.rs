use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let polymer = lines.next().unwrap().unwrap();
    lines.next();
    let mut reactions: HashMap<(char, char), char> = HashMap::new();
    let mut pairs: HashMap<(char, char), u64> = HashMap::new();
    for line in lines {
        let l = line.unwrap();
        let s: Vec<&str> = l.split(" -> ").collect();
        let mut pair = s[0].chars();
        let a = pair.next().unwrap();
        let b = pair.next().unwrap();
        reactions.insert((a, b), s[1].chars().next().unwrap());
        pairs.insert((a, b), 0);
    }

    let mut iter = polymer.chars().peekable();
    let mut a = iter.next().unwrap();
    while let Some(next) = iter.next() {
        let c = pairs.get_mut(&(a, next)).unwrap();
        *c += 1;
        a = next;
    }
    for _ in 0..40 {
        let mut next_gen: HashMap<(char, char), u64> = HashMap::new();
        for (c, count) in pairs.iter() {
            let fill = *reactions.get(c).unwrap();
            *next_gen.entry((c.0, fill)).or_default() += count;
            *next_gen.entry((fill, c.1)).or_default() += count;
        }
        pairs = next_gen;
    }

    let mut min = u64::MAX;
    let mut max = 0;
    let mut elem_counts: HashMap<char, u64> = HashMap::new();
    for (c, count) in pairs.iter() {
        *elem_counts.entry(c.0).or_default() += count;
    }
    *elem_counts.entry(a).or_default() += 1;
    for c in elem_counts.iter() {
        min = u64::min(min, *c.1);
        max = u64::max(max, *c.1);
    }
    println!("{}", max - min);
}
