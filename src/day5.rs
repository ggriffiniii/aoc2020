use aoc_runner_derive::aoc;

use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct SeatId(u16);
impl fmt::Display for SeatId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl SeatId {
    fn parse(input: &[u8]) -> Option<SeatId> {
        if input.len() != 10 {
            return None;
        }
        let mut seatid = 0u16;

        for b in &input[..7] {
            let value = match b {
                b'F' => 0,
                b'B' => 1,
                _ => return None,
            };
            seatid = (seatid << 1) | value;
        }

        for b in &input[7..10] {
            let value = match b {
                b'L' => 0,
                b'R' => 1,
                _ => return None,
            };
            seatid = (seatid << 1) | value;
        }
        Some(SeatId(seatid))
    }

    fn row(&self) -> u8 {
        (self.0 >> 3) as u8
    }

    fn column(&self) -> u8 {
        (self.0 & 0b111) as u8
    }
}

#[aoc(day5, part1)]
pub fn solve_d5_p1(input: &[u8]) -> SeatId {
    input
        .split(|&b| b == b'\n')
        .map(|i| SeatId::parse(i).unwrap())
        .max()
        .unwrap()
}
