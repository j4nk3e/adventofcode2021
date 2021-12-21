use std::{collections::HashMap, env};

const GOAL: usize = 21;
const PROB: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut states: HashMap<[usize; 4], usize> = HashMap::new();
    states.insert(
        [args[1].parse().unwrap(), 0, args[2].parse().unwrap(), 0],
        1,
    );
    let mut win1 = 0;
    let mut win2 = 0;
    let mut p1 = true;
    while !states.is_empty() {
        let mut next: HashMap<[usize; 4], usize> = HashMap::new();
        for (uni, count) in states.iter() {
            for (id, roll) in (3..10).enumerate() {
                let next_count = PROB[id] * count;
                if p1 {
                    let pos = m0d(roll + uni[0]);
                    let score = uni[1] + pos;
                    if score >= GOAL {
                        win1 += next_count;
                        continue;
                    }
                    *next.entry([pos, score, uni[2], uni[3]]).or_default() += next_count;
                } else {
                    let pos = m0d(roll + uni[2]);
                    let score = uni[3] + pos;
                    if score >= GOAL {
                        win2 += next_count;
                        continue;
                    }
                    *next.entry([uni[0], uni[1], pos, score]).or_default() += next_count;
                }
            }
        }
        p1 = !p1;
        states = next;
    }
    println!("P1 {} | P2 {}", win1, win2);
}

fn m0d(p: usize) -> usize {
    match p % 10 {
        0 => 10,
        i => i,
    }
}
