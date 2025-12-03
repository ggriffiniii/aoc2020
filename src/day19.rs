use crate::split_once;
use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[derive(Debug)]
enum Parser<'a> {
    Lit(&'a str),
    Seq(Vec<usize>),
    Alt(Vec<usize>, Vec<usize>),
}

impl<'a> Parser<'a> {
    fn new(parser_def: &'a str) -> Option<(usize, Self)> {
        let (idx, def) = split_once(parser_def, ": ")?;
        let idx = idx.parse().ok()?;
        if let Some((a, b)) = split_once(def, " | ") {
            let a = a
                .split(' ')
                .map(|x| x.parse().ok())
                .collect::<Option<Vec<usize>>>()?;
            let b = b
                .split(' ')
                .map(|x| x.parse().ok())
                .collect::<Option<Vec<usize>>>()?;
            Some((idx, Parser::Alt(a, b)))
        } else if let Some(_) = def.find('"') {
            Some((idx, Parser::Lit(&def[1..def.len() - 1])))
        } else {
            let seq = def
                .split(' ')
                .map(|x| x.parse().ok())
                .collect::<Option<Vec<usize>>>()?;
            Some((idx, Parser::Seq(seq)))
        }
    }

    // returns Some(remaining_input) when valid, None when doesn't match.
    fn parse<'b>(&self, parsers: &HashMap<usize, Parser>, input: &'b str) -> Vec<&'b str> {
        match self {
            &Parser::Lit(s) => {
                if input.is_empty() || &input[..s.len()] != s {
                    Vec::new()
                } else {
                    vec![&input[s.len()..]]
                }
            }
            Parser::Seq(a) => {
                let parser_idx = a[0];
                let p = parsers.get(&parser_idx).unwrap();
                let remaining = p.parse(parsers, input);

                let subsequent_parsers = &a[1..];
                if subsequent_parsers.is_empty() {
                    return remaining;
                }

                let mut subsequent_remaining = Vec::new();
                let subsequent_parser = Parser::Seq(subsequent_parsers.to_vec());
                for input in remaining {
                    subsequent_remaining.extend(subsequent_parser.parse(parsers, input));
                }
                subsequent_remaining
            }
            Parser::Alt(a, b) => {
                let parser_a = Parser::Seq(a.clone());
                let parser_b = Parser::Seq(b.clone());
                let mut remaining = parser_a.parse(parsers, input);
                remaining.extend(parser_b.parse(parsers, input));
                remaining
            }
        }
    }
}

#[aoc(day19, part1)]
fn solve_d19_p1(input: &str) -> usize {
    let (parser_input, pattern_input) = split_once(input, "\n\n").unwrap();
    let parsers: HashMap<usize, Parser> = parser_input
        .split('\n')
        .map(|parser_def| Parser::new(parser_def).unwrap())
        .collect();

    let rule0 = parsers.get(&0).unwrap();
    pattern_input
        .split('\n')
        .filter(|pattern| rule0.parse(&parsers, pattern).iter().any(|x| x.is_empty()))
        .count()
}

#[aoc(day19, part2)]
fn solve_d19_p2(input: &str) -> usize {
    let (parser_input, pattern_input) = split_once(input, "\n\n").unwrap();
    let mut parsers: HashMap<usize, Parser> = parser_input
        .split('\n')
        .map(|parser_def| Parser::new(parser_def).unwrap())
        .collect();

    parsers.insert(8, Parser::Alt(vec![42], vec![42, 8]));
    parsers.insert(11, Parser::Alt(vec![42, 31], vec![42, 11, 31]));

    let rule0 = parsers.get(&0).unwrap();

    pattern_input
        .split('\n')
        .filter(|pattern| rule0.parse(&parsers, pattern).iter().any(|x| x.is_empty()))
        .count()
}
