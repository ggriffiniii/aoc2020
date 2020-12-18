use aoc_runner_derive::aoc;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::map_res;
use nom::multi::fold_many0;
use nom::sequence::{delimited, pair};
use nom::IResult;
use std::str::FromStr;

fn num(i: &str) -> IResult<&str, usize> {
    map_res(delimited(space0, digit1, space0), FromStr::from_str)(i)
}

fn paren_expr(i: &str) -> IResult<&str, usize> {
    delimited(space0, delimited(tag("("), expr, tag(")")), space0)(i)
}

fn num_or_paren_expr(i: &str) -> IResult<&str, usize> {
    alt((num, paren_expr))(i)
}

fn expr(i: &str) -> IResult<&str, usize> {
    let (i, lhs) = num_or_paren_expr(i)?;

    fold_many0(
        pair(
            alt((char('+'), char('-'), char('*'), char('/'))),
            num_or_paren_expr,
        ),
        lhs,
        |lhs, (op, rhs)| match op {
            '+' => lhs + rhs,
            '-' => lhs - rhs,
            '*' => lhs * rhs,
            '/' => lhs * rhs,
            _ => unreachable!("invalid op"),
        },
    )(i)
}

#[aoc(day18, part1)]
fn solve_d18_p1(input: &str) -> usize {
    input.split('\n').map(|line| expr(line).unwrap().1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr() {
        assert_eq!(expr("2+2"), Ok(("", 4)));
        assert_eq!(expr("1 + 2 * 3 + 4 * 5 + 6"), Ok(("", 71)));
        assert_eq!(expr("1 + (2 * 3) + (4 * (5 + 6))"), Ok(("", 51)));
        assert_eq!(expr("2 * 3 + (4 * 5)"), Ok(("", 26)));
        assert_eq!(expr("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Ok(("", 437)));
        assert_eq!(
            expr("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Ok(("", 12240))
        );
        assert_eq!(
            expr("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(("", 13632))
        );
    }
}
