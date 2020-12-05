use aoc_runner_derive::aoc;

struct PasswdEntry<'a> {
    lower_bound: usize,
    upper_bound: usize,
    policy_char: u8,
    passwd: &'a [u8],
}

impl<'a> PasswdEntry<'a> {
    fn parse(mut input: &'a str) -> Option<Self> {
        let lb_idx = input.find('-')?;
        let lower_bound: usize = (&input[..lb_idx]).parse().ok()?;
        input = &input[lb_idx + 1..];

        let ub_idx = input.find(' ')?;
        let upper_bound: usize = (&input[..ub_idx]).parse().ok()?;
        input = &input[ub_idx + 1..];

        let policy_char = input.as_bytes()[0];
        let passwd = &input[3..].as_bytes();
        Some(PasswdEntry {
            lower_bound,
            upper_bound,
            policy_char,
            passwd,
        })
    }
}

#[aoc(day2, part1)]
pub fn solve_d2_p1(input: &str) -> usize {
    fn line_is_valid(line: &str) -> bool {
        let entry = PasswdEntry::parse(line).unwrap();
        let count = entry
            .passwd
            .iter()
            .filter(|&&b| b == entry.policy_char)
            .count();
        (count >= entry.lower_bound) && (count <= entry.upper_bound)
    }
    input.split('\n').filter(|x| line_is_valid(x)).count()
}

#[aoc(day2, part2)]
pub fn solve_d2_p2(input: &str) -> usize {
    fn line_is_valid(line: &str) -> bool {
        let entry = PasswdEntry::parse(line).unwrap();
        (entry.passwd[entry.lower_bound - 1] == entry.policy_char)
            ^ (entry.passwd[entry.upper_bound - 1] == entry.policy_char)
    }
    input.split('\n').filter(|x| line_is_valid(x)).count()
}
