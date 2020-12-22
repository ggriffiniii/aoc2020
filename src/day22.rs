use crate::split_once;
use aoc_runner_derive::aoc;
use std::borrow::Cow;
use std::collections::{HashSet, VecDeque};

fn deck_score(deck: VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(idx, value)| (idx + 1) * value)
        .sum()
}

#[aoc(day22, part1)]
fn solve_d22_p1(input: &str) -> usize {
    let (p1, p2) = split_once(input, "\n\n").unwrap();
    let mut p1_deck: VecDeque<usize> = p1.split('\n').skip(1).map(|x| x.parse().unwrap()).collect();
    let mut p2_deck: VecDeque<usize> = p2.split('\n').skip(1).map(|x| x.parse().unwrap()).collect();
    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        let p1_value = p1_deck.pop_front().unwrap();
        let p2_value = p2_deck.pop_front().unwrap();
        if p1_value > p2_value {
            p1_deck.push_back(p1_value);
            p1_deck.push_back(p2_value);
        } else {
            p2_deck.push_back(p2_value);
            p2_deck.push_back(p1_value);
        }
    }
    deck_score(if p1_deck.is_empty() { p2_deck } else { p1_deck })
}

enum Winner {
    Player1(VecDeque<usize>),
    Player2(VecDeque<usize>),
}

#[aoc(day22, part2)]
fn solve_d22_p2(input: &str) -> usize {
    let (p1, p2) = split_once(input, "\n\n").unwrap();
    let p1_deck: VecDeque<usize> = p1.split('\n').skip(1).map(|x| x.parse().unwrap()).collect();
    let p2_deck: VecDeque<usize> = p2.split('\n').skip(1).map(|x| x.parse().unwrap()).collect();
    match play_recursive_combat(p1_deck, p2_deck) {
        Winner::Player1(deck) | Winner::Player2(deck) => deck_score(deck),
    }
}

fn play_recursive_combat(mut p1_deck: VecDeque<usize>, mut p2_deck: VecDeque<usize>) -> Winner {
    let mut previous_rounds = HashSet::new();
    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        if previous_rounds.contains(&(Cow::Borrowed(&p1_deck), Cow::Borrowed(&p2_deck))) {
            return Winner::Player1(p1_deck);
        }
        previous_rounds.insert((Cow::Owned(p1_deck.clone()), Cow::Owned(p2_deck.clone())));

        let p1_value = p1_deck.pop_front().unwrap();
        let p2_value = p2_deck.pop_front().unwrap();
        if p1_deck.len() >= p1_value && p2_deck.len() >= p2_value {
            match play_recursive_combat(
                p1_deck.iter().copied().take(p1_value).collect(),
                p2_deck.iter().copied().take(p2_value).collect(),
            ) {
                Winner::Player1(_) => {
                    p1_deck.push_back(p1_value);
                    p1_deck.push_back(p2_value);
                }
                Winner::Player2(_) => {
                    p2_deck.push_back(p2_value);
                    p2_deck.push_back(p1_value);
                }
            }
        } else {
            if p1_value > p2_value {
                p1_deck.push_back(p1_value);
                p1_deck.push_back(p2_value);
            } else {
                p2_deck.push_back(p2_value);
                p2_deck.push_back(p1_value);
            }
        }
    }
    if p1_deck.is_empty() {
        Winner::Player2(p2_deck)
    } else {
        Winner::Player1(p1_deck)
    }
}
