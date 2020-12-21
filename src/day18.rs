use aoc_runner_derive::aoc;

fn num(i: &str) -> Option<(&str, usize)> {
    let i = i.trim_start();
    let end_idx = i
        .as_bytes()
        .iter()
        .copied()
        .position(|b| !(b'0'..=b'9').contains(&b))
        .unwrap_or(i.len());
    let n = (&i[..end_idx]).parse().ok()?;
    let rem = &i[end_idx..];
    Some((rem, n))
}

#[aoc(day18, part1)]
fn solve_d18_p1(input: &str) -> usize {
    #[derive(Debug, Copy, Clone)]
    enum Operator {
        Add,
        Mul,
    }

    fn operator(i: &str) -> Option<(&str, Operator)> {
        let i = i.trim_start();
        let op = match i.as_bytes()[0] {
            b'+' => Operator::Add,
            b'*' => Operator::Mul,
            _ => return None,
        };
        Some((&i[1..], op))
    }

    fn paren(i: &str) -> Option<(&str, usize)> {
        let i = i.trim_start();
        if i.is_empty() || i.as_bytes()[0] != b'(' {
            return None;
        }
        let (rem, n) = expr(&i[1..])?;
        if rem.is_empty() || rem.as_bytes()[0] != b')' {
            return None;
        }
        let rem = &rem[1..];
        Some((rem, n))
    }

    fn num_or_paren(i: &str) -> Option<(&str, usize)> {
        num(i).or_else(|| paren(i))
    }

    fn operator_and_rhs(i: &str) -> Option<(&str, (Operator, usize))> {
        let (i, op) = operator(i)?;
        let (i, rhs) = num_or_paren(i)?;
        Some((i, (op, rhs)))
    }

    fn expr(i: &str) -> Option<(&str, usize)> {
        let (mut rem, mut lhs) = num_or_paren(i)?;

        loop {
            if rem.is_empty() {
                break;
            }

            if let Some((irem, (op, rhs))) = operator_and_rhs(rem) {
                rem = irem;
                lhs = match op {
                    Operator::Add => lhs + rhs,
                    Operator::Mul => lhs * rhs,
                };
            } else {
                break;
            }
        }
        Some((rem, lhs))
    }

    input.split('\n').map(|line| expr(line).unwrap().1).sum()
}

#[aoc(day18, part2)]
fn solve_d18_p2(input: &str) -> usize {
    fn paren(i: &str) -> Option<(&str, usize)> {
        let i = i.trim_start();
        if i.is_empty() || i.as_bytes()[0] != b'(' {
            return None;
        }
        let (rem, n) = expr(&i[1..])?;
        if rem.is_empty() || rem.as_bytes()[0] != b')' {
            return None;
        }
        let rem = &rem[1..];
        Some((rem, n))
    }

    fn num_or_paren(i: &str) -> Option<(&str, usize)> {
        num(i).or_else(|| paren(i))
    }

    fn add_or_paren(i: &str) -> Option<(&str, usize)> {
        let (mut i, mut lhs) = num_or_paren(i)?;

        loop {
            i = i.trim_start();
            if i.is_empty() || i.as_bytes()[0] != b'+' {
                break;
            }
            i = &i[1..];
            if let Some((rem, rhs)) = num_or_paren(i) {
                lhs += rhs;
                i = rem;
            } else {
                break;
            }
        }
        Some((i, lhs))
    }

    fn expr(i: &str) -> Option<(&str, usize)> {
        let (mut i, mut lhs) = add_or_paren(i)?;

        loop {
            i = i.trim_start();
            if i.is_empty() || i.as_bytes()[0] != b'*' {
                break;
            }
            i = &i[1..];
            if let Some((rem, rhs)) = add_or_paren(i) {
                lhs *= rhs;
                i = rem;
            } else {
                break;
            }
        }
        Some((i, lhs))
    }

    input.split('\n').map(|line| expr(line).unwrap().1).sum()
}
