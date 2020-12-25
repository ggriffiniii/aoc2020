use crate::split_once;
use aoc_runner_derive::aoc;

const MOD: usize = 20201227;

#[aoc(day25, part1)]
fn solve_d25_p1(input: &str) -> usize {
    let (pubkey1, pubkey2) = split_once(input, "\n").unwrap();
    let pubkey1: usize = pubkey1.parse().unwrap();
    let pubkey2: usize = pubkey2.parse().unwrap();
    let loop_size = loop_size_from_pubkey(pubkey1);
    encryption_key_from_pubkey(pubkey2, loop_size)
}

fn encryption_key_from_pubkey(pubkey: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= pubkey;
        value %= MOD;
    }
    value
}

fn loop_size_from_pubkey(pubkey: usize) -> usize {
    let mut loop_size = 0;
    let mut value = 1;
    while value != pubkey {
        loop_size += 1;
        value *= 7;
        value %= MOD;
    }
    loop_size
}
