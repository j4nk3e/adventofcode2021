use std::io::{self, BufRead};

use bitvec::{order::Msb0, prelude::BitVec, slice::BitSlice};
use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        let mut bytes = Vec::new();
        let l = line.unwrap();
        let l = l.chars();
        for (a, b) in l.tuples() {
            bytes.push((a.to_digit(16).unwrap() << 4 | b.to_digit(16).unwrap()) as u8);
        }
        let bits: BitVec<Msb0, u8> = BitVec::from_vec(bytes);
        let val = parse(&mut 0, &bits);
        println!("{}", val);
    }
}

fn parse(pointer: &mut usize, bits: &BitVec<Msb0, u8>) -> usize {
    *pointer += 3;
    let tid: usize = slice_to_num(&bits[*pointer..*pointer + 3]);
    *pointer += 3;

    if tid == 4 {
        let mut lit = 0;
        loop {
            lit <<= 4;
            lit |= slice_to_num(&bits[*pointer + 1..*pointer + 5]);
            *pointer += 5;
            if !bits[*pointer - 5] {
                return lit;
            }
        }
    }

    let (num, size) = if bits[*pointer] {
        (true, 11)
    } else {
        (false, 15)
    };
    *pointer += size + 1;
    let len = slice_to_num(&bits[*pointer - size..*pointer]);
    let start = *pointer;
    let mut sub: Vec<usize> = Vec::new();
    loop {
        sub.push(parse(pointer, bits));
        if num && sub.len() == len || !num && *pointer >= start + len {
            break;
        }
    }
    let mut i = sub.iter();
    return match tid {
        0 => i.sum(),
        1 => i.product(),
        2 => *i.min().unwrap(),
        3 => *i.max().unwrap(),
        5 => match i.next().gt(&i.next()) {
            true => 1,
            false => 0,
        },
        6 => match i.next().lt(&i.next()) {
            true => 1,
            false => 0,
        },
        7 => match i.next().eq(&i.next()) {
            true => 1,
            false => 0,
        },
        _ => panic!("tid {} not found", tid),
    };
}

fn slice_to_num(s: &BitSlice<Msb0, u8>) -> usize {
    let mut m = 0;
    for pos in s.iter_ones() {
        m |= 1 << (s.len() - pos - 1);
    }
    m
}
