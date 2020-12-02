use aoc_runner_derive::{aoc, aoc_generator, aoc_lib};

#[aoc_generator(day1)]
pub fn d1_input(input: &str) -> Vec<u64> {
    let mut entries: Vec<u64> = input.split('\n').map(|x| x.parse().unwrap()).collect();
    entries.sort();
    entries
}

#[aoc(day1, part1)]
pub fn solve_d1_p1(input: &[u64]) -> u64 {
    for entry in input {
        let needed = 2020 - entry;
        if let Ok(idx) = input.binary_search(&needed) {
            return entry * input[idx];
        }
    }
    panic!("not found");
}

#[aoc(day1, part2)]
pub fn solve_d1_p2(input: &[u64]) -> Option<u64> {
    for i in 0..input.len() - 2 {
        let entry1 = input[i];
        for j in i + 1..input.len() - 1 {
            let entry2 = input[j];
            if entry1 + entry2 > 2020 {
                break;
            }
            for k in j + 1..input.len() {
                let entry3 = input[k];
                if entry1 + entry2 + entry3 == 2020 {
                    return Some(entry1 * entry2 * entry3);
                }
            }
        }
    }
    return None;
}

struct PasswdEntry<'a> {
    lower_bound: usize,
    upper_bound: usize,
    policy_char: u8,
    passwd: &'a [u8],
}

impl<'a> PasswdEntry<'a> {
    fn new(mut input: &'a str) -> Option<Self> {
        let lb_idx = input.find('-')?;
        let lower_bound: usize = (&input[..lb_idx]).parse().ok()?;
        input = &input[lb_idx + 1..];

        let ub_idx = input.find(' ')?;
        let upper_bound: usize = (&input[..ub_idx]).parse().ok()?;
        input = &input[ub_idx + 1..];

        let policy_char = input.as_bytes()[0];
        let passwd = &input[3..].as_bytes();
        Some(PasswdEntry { lower_bound, upper_bound, policy_char, passwd })
    }
}

#[aoc(day2, part1)]
pub fn solve_d2_p1(input: &str) -> usize {
    fn line_is_valid(line: &str) -> bool {
        let entry = PasswdEntry::new(line).unwrap();
        let count = entry.passwd.iter().filter(|&&b| b == entry.policy_char).count();
        (count >= entry.lower_bound) && (count <= entry.upper_bound)
    }
    input.split('\n').filter(|x| line_is_valid(x)).count()
}

#[aoc(day2, part2)]
pub fn solve_d2_p2(input: &str) -> usize {
    fn line_is_valid(line: &str) -> bool {
        let entry = PasswdEntry::new(line).unwrap();
        (entry.passwd[entry.lower_bound - 1] == entry.policy_char)
            ^ (entry.passwd[entry.upper_bound - 1] == entry.policy_char)
    }
    input.split('\n').filter(|x| line_is_valid(x)).count()
}

aoc_lib! { year = 2020 }
