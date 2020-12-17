use aoc_runner_derive::aoc;
use std::ops::RangeInclusive;

fn split_once<'a>(input: &'a str, delimeter: &str) -> Option<(&'a str, &'a str)> {
    let idx = input.find(delimeter)?;
    Some((&input[..idx], &input[idx + delimeter.len()..]))
}

struct Rule<'a> {
    name: &'a str,
    a: RangeInclusive<usize>,
    b: RangeInclusive<usize>,
}
impl<'a> Rule<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        let (name, rem) = split_once(input, ": ")?;
        let (a, b) = split_once(rem, " or ")?;
        let a = {
            let (start, end) = split_once(a, "-")?;
            RangeInclusive::<usize>::new(start.parse().ok()?, end.parse().ok()?)
        };
        let b = {
            let (start, end) = split_once(b, "-")?;
            RangeInclusive::<usize>::new(start.parse().ok()?, end.parse().ok()?)
        };
        Some(Rule { name, a, b })
    }

    fn matches(&self, value: usize) -> bool {
        self.a.contains(&value) || self.b.contains(&value)
    }
}

#[aoc(day16, part1)]
fn solve_d16_p1(input: &str) -> usize {
    let (rules, rem) = split_once(input, "\n\nyour ticket:\n").unwrap();
    let (_your_ticket, nearby_tickets) = split_once(rem, "\n\nnearby tickets:\n").unwrap();

    let rules: Vec<_> = rules.split('\n').map(|x| Rule::parse(x).unwrap()).collect();
    nearby_tickets
        .split('\n')
        .flat_map(|line| line.split(','))
        .filter_map(|x| {
            let value: usize = x.parse().unwrap();
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

#[test]
fn test_foo() {
    const INPUT: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
    solve_d16_p2(INPUT);
}
