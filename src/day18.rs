use aoc_runner_derive::aoc;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::map_res;
use nom::multi::fold_many0;
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;
use std::str::FromStr;

fn num(i: &str) -> IResult<&str, usize> {
    map_res(delimited(space0, digit1, space0), FromStr::from_str)(i)
}

#[aoc(day18, part1)]
fn solve_d18_p1(input: &str) -> usize {
    fn paren(i: &str) -> IResult<&str, usize> {
        delimited(space0, delimited(tag("("), expr, tag(")")), space0)(i)
    }

    fn num_or_paren(i: &str) -> IResult<&str, usize> {
        alt((num, paren))(i)
    }

    fn add_or_paren(i: &str) -> IResult<&str, usize> {
        let (i, lhs) = num_or_paren(i)?;

        fold_many0(preceded(char('+'), num_or_paren), lhs, |lhs, rhs| lhs + rhs)(i)
    }

    fn expr(i: &str) -> IResult<&str, usize> {
        let (i, lhs) = add_or_paren(i)?;

        fold_many0(
            pair(alt((char('+'), char('*'))), num_or_paren),
            lhs,
            |lhs, (op, rhs)| match op {
                '+' => lhs + rhs,
                '*' => lhs * rhs,
                _ => unreachable!(),
            },
        )(i)
    }

    input.split('\n').map(|line| expr(line).unwrap().1).sum()
}

#[aoc(day18, part2)]
fn solve_d18_p2(input: &str) -> usize {
    fn paren(i: &str) -> IResult<&str, usize> {
        delimited(space0, delimited(tag("("), expr, tag(")")), space0)(i)
    }

    fn num_or_paren(i: &str) -> IResult<&str, usize> {
        alt((num, paren))(i)
    }

    fn add_or_paren(i: &str) -> IResult<&str, usize> {
        let (i, lhs) = num_or_paren(i)?;

        fold_many0(preceded(char('+'), num_or_paren), lhs, |lhs, rhs| lhs + rhs)(i)
    }

    fn expr(i: &str) -> IResult<&str, usize> {
        let (i, lhs) = add_or_paren(i)?;

        fold_many0(preceded(char('*'), add_or_paren), lhs, |lhs, rhs| lhs * rhs)(i)
    }

    input.split('\n').map(|line| expr(line).unwrap().1).sum()
}
