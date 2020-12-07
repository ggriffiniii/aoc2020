use aoc_runner_derive::aoc;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct BagPolicy<'a> {
    color: &'a str,
    contained_bags: Vec<ContainedBags<'a>>,
}

impl<'a> BagPolicy<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        // example input:
        // "light red bags contain 1 bright white bag, 2 muted yellow bags."
        // "faded blue bags contain no other bags."
        let mut iter = input.split(" bags contain ");
        let color = iter.next()?;
        let contained = iter.next()?;
        if let Some(_) = iter.next() {
            return None;
        }
        if contained == "no other bags." {
            return Some(BagPolicy {
                color,
                contained_bags: Vec::new(),
            });
        }
        let contained_bags = contained
            .split(", ")
            .map(|i| ContainedBags::parse(i))
            .collect::<Option<Vec<_>>>()?;
        Some(BagPolicy {
            color,
            contained_bags,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ContainedBags<'a> {
    count: usize,
    color: &'a str,
}

impl<'a> ContainedBags<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        // example input: "5 faded blue bags"
        let first_space_idx = input.find(' ')?;
        let bag_color_end_idx = input.find(" bag")?;
        let count = (&input[..first_space_idx]).parse().ok()?;
        let color = &input[first_space_idx + 1..bag_color_end_idx];
        Some(ContainedBags { count, color })
    }
}

#[aoc(day7, part1)]
pub fn solve_d7_p1(input: &str) -> usize {
    // Hashmap keyed by bag color and values are a list of bags that can directly enclose that bag.
    let mut bag_graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for bag_policy in input.split('\n') {
        let bag_policy = BagPolicy::parse(bag_policy).unwrap();
        for contained_bag in bag_policy.contained_bags {
            bag_graph
                .entry(contained_bag.color)
                .or_default()
                .push(bag_policy.color);
        }
    }
    find_transitive_enclosing_bags(&bag_graph, "shiny gold").len()
}

fn find_transitive_enclosing_bags<'a>(
    graph: &HashMap<&str, Vec<&'a str>>,
    color: &str,
) -> Vec<&'a str> {
    fn _find_transitive_enclosing_bags<'a>(
        graph: &HashMap<&str, Vec<&'a str>>,
        color: &str,
        output: &mut Vec<&'a str>,
    ) {
        let enclosing_colors = match graph.get(color) {
            None => return,
            Some(enclosing_colors) => enclosing_colors,
        };
        output.extend(enclosing_colors);
        for enclosing_color in enclosing_colors {
            _find_transitive_enclosing_bags(graph, enclosing_color, output);
        }
    }
    let mut output = Vec::new();
    _find_transitive_enclosing_bags(graph, color, &mut output);
    output.sort();
    output.dedup();
    output
}

#[aoc(day7, part2)]
pub fn solve_d7_p2(input: &str) -> usize {
    let bag_policies = input
        .split('\n')
        .map(|line| {
            let policy = BagPolicy::parse(line)?;
            Some((policy.color, policy))
        })
        .collect::<Option<HashMap<_, _>>>()
        .unwrap();
    count_bags_inside(&bag_policies, "shiny gold")
}

fn count_bags_inside(graph: &HashMap<&str, BagPolicy>, color: &str) -> usize {
    let policy = graph.get(color).unwrap();
    policy
        .contained_bags
        .iter()
        .map(|contained_bag| {
            contained_bag.count * (1 + count_bags_inside(graph, contained_bag.color))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_contained_bags() {
        assert_eq!(
            ContainedBags::parse("5 faded blue bags"),
            Some(ContainedBags {
                count: 5,
                color: "faded blue"
            })
        );
    }

    #[test]
    fn parse_bag_policy() {
        assert_eq!(
            BagPolicy::parse("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            Some(BagPolicy {
                color: "light red",
                contained_bags: vec![
                    ContainedBags {
                        count: 1,
                        color: "bright white",
                    },
                    ContainedBags {
                        count: 2,
                        color: "muted yellow",
                    },
                ],
            }),
        );
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;
        assert_eq!(solve_d7_p2(INPUT), 126);
    }
}
