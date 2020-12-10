use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Jolts(usize);
impl std::ops::Add for Jolts {
    type Output = Jolts;
    fn add(self, b: Jolts) -> Jolts {
        Jolts(self.0 + b.0)
    }
}
impl std::ops::Sub for Jolts {
    type Output = Jolts;
    fn sub(self, b: Jolts) -> Jolts {
        Jolts(self.0 - b.0)
    }
}

#[aoc(day10, part1)]
pub fn solve_d10_p1(input: &str) -> usize {
    let jolts = {
        let mut jolts: Vec<Jolts> = std::iter::once(Jolts(0))
            .chain(input.split('\n').map(|x| Jolts(x.parse().unwrap())))
            .collect();
        jolts.sort();
        jolts.push(*jolts.last().unwrap() + Jolts(3)); // The devices built-in adapter is always 3 more than the highest.
        jolts
    };

    // Count the number of 1-jolt differences and 3-jolt differences in the
    // chain.
    let (j1_diff, j3_diff) =
        jolts
            .windows(2)
            .fold((0, 0), |(j1_diff, j3_diff), entry| match entry {
                &[a, b] if b - a == Jolts(1) => (j1_diff + 1, j3_diff),
                &[a, b] if b - a == Jolts(3) => (j1_diff, j3_diff + 1),
                _ => panic!("fail"),
            });
    j1_diff * j3_diff
}

#[aoc(day10, part2)]
pub fn solve_d10_p2(input: &str) -> usize {
    let jolts = {
        let mut jolts: Vec<Jolts> = std::iter::once(Jolts(0))
            .chain(input.split('\n').map(|x| Jolts(x.parse().unwrap())))
            .collect();
        jolts.sort();
        jolts.push(*jolts.last().unwrap() + Jolts(3)); // The devices built-in adapter is always 3 more than the highest.
        jolts
    };

    // Iterate over the list of jolt value for the adapters. For each entry
    // store the number of combinations this adapter has that lead back to zero
    // in `combinations_count`
    // To calculate the number of combinations the current adapter has you first
    // determine which are the potential upstream adapters and then sum their
    // combinations.

    let mut combinations_count = Vec::with_capacity(jolts.len());
    // The first entry in jolts is the `0` jolts. Initialize it with 1 to
    // indicate that it is the only way to reach the end of the chain.
    combinations_count.push(1);

    // Skip the first entry in jolts since it's already initialized in
    // combinations_count.
    for (idx, current_jolts) in jolts.iter().copied().enumerate().skip(1) {
        let mut current_combinations = 0;

        for upstream_idx in idx.saturating_sub(3)..idx {
            if current_jolts - jolts[upstream_idx] <= Jolts(3) {
                current_combinations += combinations_count[upstream_idx];
            }
        }
        combinations_count.push(current_combinations);
    }
    *combinations_count.last().unwrap()
}
