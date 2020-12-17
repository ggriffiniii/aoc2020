use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[derive(Debug)]
enum Instr<'a> {
    Mask(&'a str),
    UpdateMem { offset: u64, value: u64 },
}

impl<'a> Instr<'a> {
    fn parse(instr: &'a str) -> Option<Self> {
        let eq_idx = instr.find('=')?;
        let lhs = &instr[..eq_idx - 1];
        let rhs = &instr[eq_idx + 2..];
        if lhs == "mask" {
            Some(Instr::Mask(rhs))
        } else {
            Self::parse_mem(lhs, rhs)
        }
    }

    fn parse_mem(lhs: &str, rhs: &str) -> Option<Self> {
        let offset: u64 = lhs[4..lhs.len() - 1].parse().ok()?;
        let value: u64 = rhs.parse().ok()?;
        Some(Instr::UpdateMem { offset, value })
    }
}

#[derive(Debug)]
struct Mask {
    or_mask: u64,
    and_mask: u64,
}
impl Mask {
    fn parse(mask: &[u8]) -> Option<Mask> {
        if mask.len() != 36 {
            return None;
        }
        let mut and_mask = !0;
        let mut or_mask = 0;
        for (idx, b) in mask.iter().enumerate() {
            match b {
                b'0' => {
                    and_mask &= !(1 << 35 - idx);
                }
                b'1' => {
                    or_mask |= 1 << 35 - idx;
                }
                b'X' => {}
                _ => return None,
            }
        }
        Some(Mask { and_mask, or_mask })
    }

    fn mask(&self, value: u64) -> u64 {
        value & self.and_mask | self.or_mask
    }
}

#[aoc(day14, part1)]
fn solve_d14_p1(input: &str) -> u64 {
    // Assume initialization doesn't write to any memory offset multiple times.
    let mut iter = input.split('\n').map(|x| Instr::parse(x).unwrap());
    let mut mask = match iter.next() {
        Some(Instr::Mask(mask)) => Mask::parse(mask.as_bytes()).unwrap(),
        _ => panic!("first line is not valid bitmask"),
    };
    let mut memory = std::collections::HashMap::new();
    for instr in iter {
        match instr {
            Instr::Mask(new_mask) => mask = Mask::parse(new_mask.as_bytes()).unwrap(),
            Instr::UpdateMem { offset, value } => {
                memory.insert(offset, mask.mask(value));
            }
        }
    }
    memory.values().sum()
}

struct Mask2 {
    floating: u64,
    or_mask: u64,
}

impl Mask2 {
    fn parse(mask: &[u8]) -> Option<Mask2> {
        if mask.len() != 36 {
            return None;
        }
        let mut floating = 0;
        let mut or_mask = 0;
        for (idx, b) in mask.iter().enumerate() {
            match b {
                b'0' => {}
                b'1' => {
                    or_mask |= 1 << 35 - idx;
                }
                b'X' => {
                    floating |= 1 << 35 - idx;
                }
                _ => return None,
            }
        }
        Some(Mask2 { floating, or_mask })
    }

    fn set_memory(&self, mem: &mut HashMap<u64, u64>, address: u64, value: u64) {
        let offset = address | self.or_mask;
        for mut floating_value in 0..2 << self.floating.count_ones() {
            let mut addr = offset;
            for bit in BitIndexes::new(self.floating) {
                addr = set_bit(addr, bit, (floating_value & 1) == 1);
                floating_value >>= 1;
            }
            mem.insert(addr, value);
        }
    }
}

fn set_bit(value: u64, bit_idx: u64, enabled: bool) -> u64 {
    if enabled {
        value | (1 << bit_idx)
    } else {
        value & !(1 << bit_idx)
    }
}

struct BitIndexes {
    value: u64,
    current_idx: u64,
}
impl BitIndexes {
    fn new(value: u64) -> Self {
        BitIndexes {
            value,
            current_idx: 0,
        }
    }
}

impl Iterator for BitIndexes {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        while self.value > 0 {
            let set = self.value & 1 == 1;
            self.value >>= 1;
            self.current_idx += 1;
            if set {
                return Some(self.current_idx - 1);
            }
        }
        return None;
    }
}

#[aoc(day14, part2)]
fn solve_d14_p2(input: &str) -> u64 {
    let mut iter = input.split('\n').map(|x| Instr::parse(x).unwrap());
    let mut mask = match iter.next() {
        Some(Instr::Mask(mask)) => Mask2::parse(mask.as_bytes()).unwrap(),
        _ => panic!("first line is not valid bitmask"),
    };
    let mut memory = std::collections::HashMap::new();
    for instr in iter {
        match instr {
            Instr::Mask(new_mask) => mask = Mask2::parse(new_mask.as_bytes()).unwrap(),
            Instr::UpdateMem { offset, value } => {
                mask.set_memory(&mut memory, offset, value);
            }
        }
    }
    memory.values().sum()
}
