use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut caves: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let l = line.unwrap();
        let conn: Vec<String> = l.split("-").map(str::to_string).collect();
        for cave in conn.iter() {
            let other = conn.iter().filter(|c| *c != "start" && *c != cave).next();
            let c = caves.entry(cave.to_string()).or_insert(Vec::new());
            if let Some(o) = other {
                c.push(o.to_string());
            }
        }
    }

    let count = find_paths(&caves, "start".to_string(), "start".to_string(), false);
    println!("{}", count);
}

fn find_paths(caves: &HashMap<String, Vec<String>>, path: String, pos: String, twice: bool) -> u64 {
    if pos == "end" {
        return 1;
    }
    let mut count = 0;
    for p in caves.get(&pos).unwrap().iter() {
        let prev = p.chars().next().unwrap().is_lowercase() && path.contains(p);
        if prev && twice {
            continue;
        }
        let new_path = path.to_owned() + "-" + p;
        count += find_paths(caves, new_path, p.to_string(), prev || twice);
    }
    return count;
}
