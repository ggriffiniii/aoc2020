use aoc_runner_derive::aoc;
use std::ops::RangeInclusive;

fn split_once<'a>(input: &'a str, delimeter: &str) -> Option<(&'a str, &'a str)> {
    let idx = input.find(delimeter)?;
    Some((&input[..idx], &input[idx + delimeter.len()..]))
}

struct Rule<'a> {
    name: &'a str,
    a: RangeInclusive<u16>,
    b: RangeInclusive<u16>,
}
impl<'a> Rule<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        let (name, rem) = split_once(input, ": ")?;
        let (a, b) = split_once(rem, " or ")?;
        let a = {
            let (start, end) = split_once(a, "-")?;
            RangeInclusive::<u16>::new(start.parse().ok()?, end.parse().ok()?)
        };
        let b = {
            let (start, end) = split_once(b, "-")?;
            RangeInclusive::<u16>::new(start.parse().ok()?, end.parse().ok()?)
        };
        Some(Rule { name, a, b })
    }

    fn matches(&self, value: u16) -> bool {
        self.a.contains(&value) || self.b.contains(&value)
    }
}

#[aoc(day16, part1)]
fn solve_d16_p1(input: &str) -> u16 {
    let (rules, rem) = split_once(input, "\n\nyour ticket:\n").unwrap();
    let (_your_ticket, nearby_tickets) = split_once(rem, "\n\nnearby tickets:\n").unwrap();

    let rules: Vec<_> = rules.split('\n').map(|x| Rule::parse(x).unwrap()).collect();
    nearby_tickets
        .split('\n')
        .flat_map(|line| line.split(','))
        .filter_map(|x| {
            let value: u16 = x.parse().unwrap();
            if rules.iter().any(|rule| rule.matches(value)) {
                None
            } else {
                Some(value)
            }
        })
        .sum()
}

#[aoc(day16, part2)]
fn solve_d16_p2(input: &str) -> usize {
    let (rules, rem) = split_once(input, "\n\nyour ticket:\n").unwrap();
    let (my_ticket, nearby_tickets) = split_once(rem, "\n\nnearby tickets:\n").unwrap();

    let rules: Vec<_> = rules.split('\n').map(|x| Rule::parse(x).unwrap()).collect();
    assert!(rules.len() < 63);
    let mut candidates = vec![(1u64 << rules.len()) - 1; rules.len()];
    let mut scratch = Vec::with_capacity(rules.len());
    for line in nearby_tickets.split('\n') {
        scratch.clear();
        scratch.extend(line.split(',').map(|field| {
            let value = field.parse().unwrap();
            // Initialize a bitmap of which rules the field is valid for. `1`
            // indicates the value is valid for that field. `0` is invalid.
            let mut bitmap = 0u64;
            for (rule_idx, rule) in rules.iter().enumerate() {
                if rule.matches(value) {
                    bitmap |= 1 << rule_idx;
                }
            }
            bitmap
        }));
        if scratch.iter().copied().any(|x| x == 0) {
            continue;
        }
        candidates
            .iter_mut()
            .zip(scratch.iter())
            .for_each(|(candidate, valid_bitmask)| {
                *candidate &= valid_bitmask;
            });
    }
    while candidates.iter().copied().any(|x| x.count_ones() > 1) {
        for idx in 0..candidates.len() {
            let candidate = candidates[idx];
            if candidate.count_ones() == 1 {
                let mask = !candidate;
                for before in &mut candidates[..idx] {
                    *before &= mask;
                }
                for after in &mut candidates[idx + 1..] {
                    *after &= mask;
                }
            }
        }
    }
    my_ticket
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .zip(candidates.into_iter().map(|x| x.trailing_zeros() as usize))
        .filter_map(|(field, rule_idx)| {
            if rules[rule_idx].name.starts_with("departure") {
                Some(field)
            } else {
                None
            }
        })
        .product()
}

#[aoc(day16, part2, avx2)]
fn solve_d16_p2_avx2(input: &str) -> usize {
    unsafe { avx2::solve_d16_p2(input) }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
mod avx2 {
    use super::{split_once, Rule};
    use std::arch::x86_64::*;

    struct RuleEval {
        lo_start: [__m256i; 2],
        lo_end: [__m256i; 2],
        hi_start: [__m256i; 2],
        hi_end: [__m256i; 2],
    }

    impl RuleEval {
        #[target_feature(enable = "avx2")]
        unsafe fn new(rules: &[Rule]) -> Option<Self> {
            if rules.len() > 31 {
                return None;
            }
            let lo_start = {
                let mut lo_start = [_mm256_setzero_si256(); 2];
                let lo_start_16 = &mut *(&mut lo_start as *mut [__m256i; 2] as *mut [i16; 32]);
                let mut rules_iter = rules.iter();
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    lo_start_16[idx] = *rule.a.start() as i16 - 1;
                }
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    lo_start_16[idx + 16] = *rule.a.start() as i16 - 1;
                }
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    lo_start_16[idx + 8] = *rule.a.start() as i16 - 1;
                }
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    lo_start_16[idx + 24] = *rule.a.start() as i16 - 1;
                }
                lo_start
            };
            let lo_end = {
                let mut lo_end = [_mm256_setzero_si256(); 2];
                let lo_end_16 = &mut *(&mut lo_end as *mut [__m256i; 2] as *mut [i16; 32]);
                let mut rules_iter = rules.iter();
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    lo_end_16[idx] = *rule.a.end() as i16 + 1;
                }
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    lo_end_16[idx + 16] = *rule.a.end() as i16 + 1;
                }
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    lo_end_16[idx + 8] = *rule.a.end() as i16 + 1;
                }
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    lo_end_16[idx + 24] = *rule.a.end() as i16 + 1;
                }
                lo_end
            };
            let hi_start = {
                let mut hi_start = [_mm256_setzero_si256(); 2];
                let hi_start_16 = &mut *(&mut hi_start as *mut [__m256i; 2] as *mut [i16; 32]);
                let mut rules_iter = rules.iter();
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    hi_start_16[idx] = *rule.b.start() as i16 - 1;
                }
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    hi_start_16[idx + 16] = *rule.b.start() as i16 - 1;
                }
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    hi_start_16[idx + 8] = *rule.b.start() as i16 - 1;
                }
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    hi_start_16[idx + 24] = *rule.b.start() as i16 - 1;
                }
                hi_start
            };
            let hi_end = {
                let mut hi_end = [_mm256_setzero_si256(); 2];
                let hi_end_16 = &mut *(&mut hi_end as *mut [__m256i; 2] as *mut [i16; 32]);
                let mut rules_iter = rules.iter();
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    hi_end_16[idx] = *rule.b.end() as i16 + 1;
                }
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    hi_end_16[idx + 16] = *rule.b.end() as i16 + 1;
                }
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    hi_end_16[idx + 8] = *rule.b.end() as i16 + 1;
                }
                for (idx, rule) in rules_iter.by_ref().take(8).enumerate() {
                    hi_end_16[idx + 24] = *rule.b.end() as i16 + 1;
                }
                hi_end
            };
            Some(RuleEval {
                lo_start,
                lo_end,
                hi_start,
                hi_end,
            })
        }

        #[target_feature(enable = "avx2")]
        unsafe fn eval(&self, value: u16) -> u32 {
            let value = _mm256_set1_epi16(value as i16);
            let within_lo = [
                _mm256_and_si256(
                    _mm256_cmpgt_epi16(self.lo_end[0], value),
                    _mm256_cmpgt_epi16(value, self.lo_start[0]),
                ),
                _mm256_and_si256(
                    _mm256_cmpgt_epi16(self.lo_end[1], value),
                    _mm256_cmpgt_epi16(value, self.lo_start[1]),
                ),
            ];
            let within_hi = [
                _mm256_and_si256(
                    _mm256_cmpgt_epi16(self.hi_end[0], value),
                    _mm256_cmpgt_epi16(value, self.hi_start[0]),
                ),
                _mm256_and_si256(
                    _mm256_cmpgt_epi16(self.hi_end[1], value),
                    _mm256_cmpgt_epi16(value, self.hi_start[1]),
                ),
            ];
            let valid = [
                _mm256_or_si256(within_lo[0], within_hi[0]),
                _mm256_or_si256(within_lo[1], within_hi[1]),
            ];
            let packed = _mm256_packs_epi16(valid[0], valid[1]);
            _mm256_movemask_epi8(packed) as u32
        }
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn solve_d16_p2(input: &str) -> usize {
        let (rules, rem) = split_once(input, "\n\nyour ticket:\n").unwrap();
        let (my_ticket, nearby_tickets) = split_once(rem, "\n\nnearby tickets:\n").unwrap();

        let rules: Vec<_> = rules.split('\n').map(|x| Rule::parse(x).unwrap()).collect();
        assert!(rules.len() < 32);
        let rule_eval = RuleEval::new(&rules).unwrap();
        let mut candidates = [_mm256_set1_epi32((1i32 << rules.len()) - 1); 4];
        let mut scratch_space = [_mm256_set1_epi32(1); 4];
        for line in nearby_tickets.split('\n') {
            let scratch_slice = &mut * (&mut scratch_space as *mut _ as *mut [u32; 32]);
            for (field, scratch) in line.split(',').zip(scratch_slice.iter_mut()) {
                let field = field.parse().unwrap();
                *scratch = rule_eval.eval(field);
            }

            if scratch_space.iter().copied().any(|elem| {
                _mm256_movemask_epi8(_mm256_cmpeq_epi32(elem, _mm256_set1_epi32(0))) != 0
            }) {
                continue;
            }

            for (candidate, valid_bitmask) in candidates.iter_mut().zip(scratch_space.iter()) {
                *candidate = _mm256_and_si256(*candidate, *valid_bitmask);
            }
        }

        let candidates = &mut * (&mut candidates as *mut _ as *mut [u32; 32]);
        let candidates = &mut candidates[..rules.len()];
        while candidates.iter().copied().any(|x| x.count_ones() > 1) {
            for idx in 0..candidates.len() {
                let candidate = candidates[idx];
                if candidate.count_ones() == 1 {
                    let mask = !candidate;
                    for before in &mut candidates[..idx] {
                        *before &= mask;
                    }
                    for after in &mut candidates[idx + 1..] {
                        *after &= mask;
                    }
                }
            }
        }
        my_ticket
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .zip(candidates.into_iter().map(|x| x.trailing_zeros() as usize))
            .filter_map(|(field, rule_idx)| {
                if rules[rule_idx].name.starts_with("departure") {
                    Some(field)
                } else {
                    None
                }
            })
            .product()
    }
}
