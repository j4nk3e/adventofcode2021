use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut count = 0;
    for line in lines {
        let re = Regex::new(r"^(.*) \| (.*)$").unwrap();
        let line = line.unwrap();
        let groups = re.captures(line.trim()).unwrap();
        let mut vars: Vec<u8> = groups
            .get(1)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|s| seg_to_int(s))
            .collect();
        let mut mapping: HashMap<u8, u8> = HashMap::new();
        vars.sort_by_key(|n| bit_count(*n));
        mapping.insert(1, vars.remove(0));
        mapping.insert(7, vars.remove(0));
        mapping.insert(4, vars.remove(0));
        mapping.insert(8, vars.remove(6));
        let p = vars
            .iter()
            .skip(3)
            .position(|n| bit_count(mapping[&7] & *n) != 3)
            .unwrap();
        mapping.insert(6, vars.remove(p + 3));
        let p = vars
            .iter()
            .position(|n| bit_count((mapping[&7] ^ *n) & *n) == 2)
            .unwrap();
        mapping.insert(3, vars.remove(p));
        let p = vars
            .iter()
            .position(|n| bit_count((mapping[&6] ^ *n) & mapping[&6]) == 1)
            .unwrap();
        mapping.insert(5, vars.remove(p));
        mapping.insert(2, vars.remove(0));
        let p = vars
            .iter()
            .position(|n| bit_count(mapping[&4] | *n) == 7)
            .unwrap();
        mapping.insert(0, vars.remove(p));
        mapping.insert(9, vars.remove(0));

        let nums: Vec<u8> = groups
            .get(2)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|s| seg_to_int(s))
            .collect();

        let mut num = 0;
        for n in nums.iter().map(|n| mapping.iter().find(|(_, v)| *v == n)) {
            num *= 10;
            num += *n.unwrap().0 as u64;
        }
        count += num;
    }
    println!("{}", count);
}

fn seg_to_int(seg: &str) -> u8 {
    let mut i = 0u8;
    for char in "abcdefg".chars() {
        i = i << 1;
        if seg.contains(char) {
            i += 1;
        }
    }
    return i;
}

fn bit_count(val: u8) -> u8 {
    let mut i = 0u8;
    for n in 0..8 {
        i += (val >> n) & 1;
    }
    return i;
}
