use std::io::{self, BufRead};

use bitvec::{order::Msb0, prelude::BitVec, slice::BitSlice};
use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        let mut bytes = Vec::new();
        let l = line.unwrap();
        let l = l.chars().into_iter();
        for (a, b) in l.tuples() {
            bytes.push((a.to_digit(16).unwrap() << 4 | b.to_digit(16).unwrap()) as u8);
        }
        let bits: BitVec<Msb0, u8> = BitVec::from_vec(bytes);
        println!("{}", bits);
        let ver = parse(&mut 0, &bits);
        println!("{}", ver);
    }
}

fn parse(pointer: &mut usize, bits: &BitVec<Msb0, u8>) -> u32 {
    let mut ver: u32 = slice_to_num(&bits[*pointer..*pointer + 3]);
    *pointer += 3;
    let tid: u32 = slice_to_num(&bits[*pointer..*pointer + 3]);
    *pointer += 3;
    let mut lit = 0;
    if tid == 4 {
        loop {
            lit <<= 4;
            *pointer += 5;
            lit |= slice_to_num(&bits[*pointer - 4..*pointer]);
            if !bits[*pointer - 5] {
                break;
            }
        }
        println!(">>> v{} {} '{}' ", ver, pointer, lit);
    } else {
        let size = if bits[*pointer] { 11 } else { 15 };
        *pointer += size + 1;
        let start = pointer.clone();
        let len = slice_to_num(&bits[*pointer - size..*pointer]);
        while *pointer - start < len as usize {
            ver += parse(pointer, bits);
        }
    }
    return ver;
}

fn slice_to_num(s: &BitSlice<Msb0, u8>) -> u32 {
    let mut m = 0;
    for pos in s.iter_ones() {
        m |= 1 << (s.len() - pos - 1);
    }
    return m;
}
