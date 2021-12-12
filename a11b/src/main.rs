use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut field: HashMap<(i8, i8), u8> = HashMap::new();
    let mut y = 0i8;
    for line in lines {
        let line = line.unwrap();
        let mut x = 0i8;
        for c in line.chars() {
            let h = c as u8 - b'0';
            field.insert((y, x), h);
            x += 1;
        }
        y += 1;
    }
    let mut i = 0;
    loop {
        i += 1;
        let mut c = 0;
        for (_, level) in field.iter_mut() {
            *level += 1;
        }

        loop {
            let fc = field.clone();
            let mut change = false;
            for (pos, _) in fc.iter().filter(|(_, v)| **v == 10) {
                change = true;
                *field.get_mut(pos).unwrap() += 1;
                for z in [
                    (pos.0 - 1, pos.1),
                    (pos.0 + 1, pos.1),
                    (pos.0, pos.1 - 1),
                    (pos.0, pos.1 + 1),
                    (pos.0 - 1, pos.1 - 1),
                    (pos.0 + 1, pos.1 + 1),
                    (pos.0 + 1, pos.1 - 1),
                    (pos.0 - 1, pos.1 + 1),
                ] {
                    match field.get_mut(&z) {
                        Some(m) => {
                            if *m < 10 {
                                *m += 1
                            }
                        }
                        _ => (),
                    }
                }
            }
            if !change {
                break;
            }
        }

        for (_, level) in field.iter_mut() {
            if *level > 9 {
                c += 1;
                *level = 0;
            }
        }
        if c == 100 {
            println!("sync {}", i);
            break;
        }
    }
}
