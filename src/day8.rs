use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone)]
enum Instr {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Instr {
    fn parse(input: &str) -> Option<Instr> {
        let space_idx = input.find(' ')?;
        let (op, arg) = (&input[..space_idx], &input[space_idx + 1..]);
        let arg = arg.parse().ok()?;
        Some(match op {
            "nop" => Instr::Nop(arg),
            "acc" => Instr::Acc(arg),
            "jmp" => Instr::Jmp(arg),
            _ => return None,
        })
    }
}

#[derive(Debug, Clone)]
struct BitSet(Vec<u64>);

impl BitSet {
    fn new(max_bits: usize) -> Self {
        BitSet(vec![0; (max_bits - 1) / 64 + 1])
    }

    fn contains(&self, bit_idx: usize) -> bool {
        let byte_idx = bit_idx / 64;
        let bit_offset = bit_idx & 63;
        self.0[byte_idx] & (1 << bit_offset) != 0
    }

    fn insert(&mut self, bit_idx: usize) {
        let byte_idx = bit_idx / 64;
        let bit_offset = bit_idx & 63;
        self.0[byte_idx] |= 1 << bit_offset;
    }
}

fn run_program(
    program: &[Instr],
    mut prev_instrs: BitSet,
    mut pc: usize,
    mut accum: isize,
) -> (usize, isize) {
    while pc < program.len() {
        if prev_instrs.contains(pc) {
            return (pc, accum);
        }
        let (new_pc, new_accum) = step(program[pc], &mut prev_instrs, pc, accum);
        pc = new_pc;
        accum = new_accum;
    }
    (pc, accum)
}

fn step(
    instr: Instr,
    prev_instrs: &mut BitSet,
    mut pc: usize,
    mut accum: isize,
) -> (usize, isize) {
    prev_instrs.insert(pc);
    pc = match instr {
        Instr::Nop(_) => pc + 1,
        Instr::Acc(arg) => {
            accum += arg;
            pc + 1
        }
        Instr::Jmp(arg) => (pc as isize + arg) as usize,
    };
    (pc, accum)
}

#[aoc(day8, part1)]
pub fn solve_d8_p1(input: &str) -> isize {
    let program = input
        .split('\n')
        .map(|line| Instr::parse(line))
        .collect::<Option<Vec<_>>>()
        .unwrap();
    run_program(&program, BitSet::new(program.len()), 0, 0).1
}

#[aoc(day8, part2)]
pub fn solve_d8_p2(input: &str) -> isize {
    let mut program = input
        .split('\n')
        .map(|line| Instr::parse(line))
        .collect::<Option<Vec<_>>>()
        .unwrap();
    let mut pc = 0;
    let mut accum = 0;
    let mut prev_instrs = BitSet::new(program.len());
    loop {
        match program[pc] {
            Instr::Nop(arg) => {
                program[pc] = Instr::Jmp(arg);
                let (new_pc, new_accum) = run_program(&program, prev_instrs.clone(), pc, accum);
                if new_pc == program.len() {
                    return new_accum;
                }
                program[pc] = Instr::Nop(arg);
            }
            Instr::Jmp(arg) => {
                program[pc] = Instr::Nop(arg);
                let (new_pc, new_accum) = run_program(&program, prev_instrs.clone(), pc, accum);
                if new_pc == program.len() {
                    return new_accum;
                }
                program[pc] = Instr::Jmp(arg);
            }
            _ => {}
        }
        let (new_pc, new_accum) = step(program[pc], &mut prev_instrs, pc, accum);
        pc = new_pc;
        accum = new_accum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitset() {
        let mut bs = BitSet::new(192);
        for bit in 0..191 {
            assert!(!bs.contains(bit));
        }
        bs.insert(5);
        assert!(bs.contains(5));
        bs.insert(191);
        assert!(bs.contains(191));
        bs.insert(63);
        assert!(bs.contains(63));
        bs.insert(64);
        assert!(bs.contains(64));
        bs.insert(65);
        assert!(bs.contains(65));
    }
}
