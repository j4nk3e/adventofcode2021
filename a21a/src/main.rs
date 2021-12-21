fn main() {
    let mut s1 = 5;
    let mut s2 = 6;
    let mut score1 = 0;
    let mut score2 = 0;
    let mut d = 0;
    let mut rounds = 0;
    loop {
        for _ in 0..3 {
            d = d + 1;
            s1 = m0d(s1 + d);
        }
        score1 += s1;
        rounds += 1;
        if score1 >= 1000 {
            println!("{}", score2 * &rounds * 3);
            break;
        }

        for _ in 0..3 {
            d = d + 1;
            s2 = m0d(s2 + d);
        }
        score2 += s2;
        rounds += 1;
        if score2 >= 1000 {
            println!("{}", score1 * &rounds * 3);
            break;
        }
        d = m0d(d);
    }
    println!("P1 - {} {} {}", d, s1, score1);
    println!("P2 - {} {} {}", d, s2, score2);
}

fn m0d(p: i32) -> i32 {
    match p % 10 {
        0 => 10,
        i => i,
    }
}
