use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut scores: Vec<u64> = stdin
        .lock()
        .lines()
        .filter_map(|line| {
            let mut stack: VecDeque<char> = VecDeque::new();
            for c in line.unwrap().chars() {
                match c {
                    '(' => stack.push_back(')'),
                    '[' => stack.push_back(']'),
                    '{' => stack.push_back('}'),
                    '<' => stack.push_back('>'),
                    _ => match stack.pop_back() {
                        None => return None,
                        Some(t) => {
                            if t != c {
                                return None;
                            }
                        }
                    },
                }
            }
            let mut x = 0u64;
            while let Some(c) = stack.pop_back() {
                x *= 5;
                x += match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => return None,
                }
            }
            Some(x)
        })
        .collect();
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
    Ok(())
}
