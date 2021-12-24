use std::{
    cell::RefCell,
    io::{self, BufRead},
    panic, usize,
};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    lines.next();
    lines.next();
    let top: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let bottom: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let mut rooms: Vec<Room> = Vec::new();
    rooms.push(Room::o(0));
    rooms.push(Room::o(1));
    rooms.push(Room {
        i: 2,
        dest: Some('A'),
        capacity: 4,
        space: RefCell::new([bottom[3], 'D', 'D', top[3]].to_vec()),
    });
    rooms.push(Room::o(3));
    rooms.push(Room {
        i: 4,
        dest: Some('B'),
        capacity: 4,
        space: RefCell::new([bottom[5], 'B', 'C', top[5]].to_vec()),
    });
    rooms.push(Room::o(5));
    rooms.push(Room {
        i: 6,
        dest: Some('C'),
        capacity: 4,
        space: RefCell::new([bottom[7], 'A', 'B', top[7]].to_vec()),
    });
    rooms.push(Room::o(7));
    rooms.push(Room {
        i: 8,
        dest: Some('D'),
        capacity: 4,
        space: RefCell::new([bottom[9], 'C', 'A', top[9]].to_vec()),
    });
    rooms.push(Room::o(9));
    rooms.push(Room::o(10));
    let cost = solve(rooms);
    println!("{}", cost);
}

fn solve(rooms: Vec<Room>) -> usize {
    let mut cost = 0;
    loop {
        let mut change = false;
        let finisher: Vec<&Room> = rooms.iter().filter(|r| r.blocked()).collect();
        'fin: for s in finisher.iter() {
            let c = s.peek();
            let target = match c {
                'A' => 2,
                'B' => 4,
                'C' => 6,
                'D' => 8,
                _ => panic!(),
            };
            let t = &rooms[target];
            if t.finishing() {
                for j in if t.i < s.i { t.i..s.i } else { s.i + 1..t.i } {
                    if rooms[j].blocked() {
                        continue 'fin;
                    }
                }
                change = true;
                cost += (t.i as isize - s.i as isize).abs() as usize * factor(c);
                let (c, _) = s.pop();
                cost += t.push(c);
            }
        }
        if !change {
            break;
        }
    }
    if rooms.iter().all(|r| r.complete()) {
        return cost;
    }
    [2, 4, 6, 8]
        .iter()
        .map(|starter| {
            let r = &rooms[*starter];
            if r.finishing() {
                return usize::MAX;
            }
            let mut targets: Vec<&Room> = Vec::new();
            for left in (0..*starter + 1).rev() {
                if rooms[left].blocked() {
                    break;
                } else if rooms[left].hall() {
                    targets.push(&rooms[left]);
                }
            }
            for right in *starter + 1..rooms.len() {
                if rooms[right].blocked() {
                    break;
                } else if rooms[right].hall() {
                    targets.push(&rooms[right]);
                }
            }
            if targets.is_empty() {
                return usize::MAX;
            }
            targets
                .iter()
                .map(|t| {
                    let next_step = rooms.clone();
                    let (c, pop) = next_step[*starter].pop();
                    next_step[t.i].push(c);
                    match solve(next_step) {
                        usize::MAX => usize::MAX,
                        n @ _ => {
                            n + cost
                                + (t.i as isize - *starter as isize).abs() as usize * factor(c)
                                + pop
                        }
                    }
                })
                .min()
                .unwrap_or(usize::MAX)
        })
        .min()
        .unwrap_or(usize::MAX)
}

fn factor(c: char) -> usize {
    match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!(),
    }
}

#[derive(Clone, Debug)]
struct Room {
    i: usize,
    dest: Option<char>,
    capacity: usize,
    space: RefCell<Vec<char>>,
}

impl Room {
    fn o(i: usize) -> Room {
        Room {
            i,
            dest: None,
            capacity: 1,
            space: RefCell::new(Vec::with_capacity(1)),
        }
    }

    fn empty(&self) -> bool {
        self.space.borrow().is_empty()
    }

    fn hall(&self) -> bool {
        self.dest == None
    }

    fn blocked(&self) -> bool {
        self.dest == None && !self.space.borrow().is_empty()
    }

    fn pop(&self) -> (char, usize) {
        let c = self.space.borrow_mut().pop().unwrap();
        (c, factor(c) * (self.capacity - self.space.borrow().len()))
    }

    fn push(&self, c: char) -> usize {
        self.space.borrow_mut().push(c);
        factor(c) * (1 + self.capacity - self.space.borrow().len())
    }

    fn peek(&self) -> char {
        *self.space.borrow_mut().last().unwrap()
    }

    fn finishing(&self) -> bool {
        self.space.borrow().iter().all(|c| Some(*c) == self.dest)
    }

    fn complete(&self) -> bool {
        match self.dest {
            None => self.empty(),
            Some(d) => {
                self.space.borrow().len() == self.capacity
                    && self.space.borrow().iter().all(|c| *c == d)
            }
        }
    }
}
