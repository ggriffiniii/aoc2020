use aoc_runner_derive::{aoc_lib, aoc};

#[aoc(day1, part1)]
pub fn solve_d1_p1(input: &str) -> u64 {
    let mut entries: Vec<u64> = input.split('\n').map(|x| x.parse().unwrap()).collect();
    entries.sort();
    for entry in &entries {
        let needed = 2020 - entry;
        if let Ok(idx) = entries.binary_search(&needed) {
            return entry * entries[idx];
        }
    }
    panic!("not found");
}

#[aoc(day1, part2)]
pub fn solve_d1_p2(input: &str) -> Option<u64> {
    let mut entries: Vec<u64> = input.split('\n').map(|x| x.parse().unwrap()).collect();
    entries.sort();
    for i in 0 .. entries.len()-2 {
        let entry1 = entries[i];
        for j in i+1 .. entries.len()-1 {
            let entry2 = entries[j];
            if entry1 + entry2 > 2020 {
                break;
            }
            for k in j+1 .. entries.len() {
                let entry3 = entries[k];
                if entry1 + entry2 + entry3 == 2020 {
                    return Some(entry1 * entry2 * entry3)
                }
            }
        }
    }
    return None
}

#[aoc(day2, part1)]
pub fn solve_d2_p1(input: &str) -> usize {
    fn line_is_valid(mut line: &str) -> bool {
        let lb_idx = line.find('-').unwrap();
        let lb: usize = (&line[..lb_idx]).parse().unwrap();
        line = &line[lb_idx+1..];

        let ub_idx = line.find(' ').unwrap();
        let ub: usize = (&line[..ub_idx]).parse().unwrap();
        line = &line[ub_idx+1..];

        let letter = line.as_bytes()[0];
        let passwd = &line[3..];
        let count = passwd.bytes().filter(|&b| b == letter).count();
        count >= lb && count <= ub
    }
    input.split('\n').filter(|x| line_is_valid(x)).count()
}

#[aoc(day2, part2)]
pub fn solve_d2_p2(input: &str) -> usize {
    fn line_is_valid(mut line: &str) -> bool {
        let lb_idx = line.find('-').unwrap();
        let lb: usize = (&line[..lb_idx]).parse().unwrap();
        line = &line[lb_idx+1..];

        let ub_idx = line.find(' ').unwrap();
        let ub: usize = (&line[..ub_idx]).parse().unwrap();
        line = &line[ub_idx+1..];

        let letter = line.as_bytes()[0];
        let passwd = &line[3..].as_bytes();
        (passwd[lb-1] == letter) ^ (passwd[ub-1] == letter)
    }
    input.split('\n').filter(|x| line_is_valid(x)).count()
}

aoc_lib!{ year = 2020 }