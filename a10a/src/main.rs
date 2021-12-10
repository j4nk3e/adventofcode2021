use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let score: u64 = lines
        .map(|line| {
            let mut stack: VecDeque<char> = VecDeque::new();
            for c in line.unwrap().chars() {
                match c {
                    '(' => stack.push_back(')'),
                    '[' => stack.push_back(']'),
                    '{' => stack.push_back('}'),
                    '<' => stack.push_back('>'),
                    _ => match stack.pop_back() {
                        None => return 0,
                        Some(t) => {
                            if t != c {
                                return match c {
                                    ')' => 3,
                                    ']' => 57,
                                    '}' => 1197,
                                    _ => 25137,
                                };
                            }
                        }
                    },
                }
            }
            return 0;
        })
        .sum();
    println!("{}", score);
    Ok(())
}
