use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LeftRight {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    fn xy(self) -> (isize, isize) {
        match self {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        }
    }

    fn from_usize(x: usize) -> Self {
        match x {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("invalid direction"),
        }
    }

    fn rotate_ship(self, degrees: usize, lr: LeftRight) -> Self {
        debug_assert!(degrees % 90 == 0);
        let num_turns = degrees / 90;
        Direction::from_usize(match lr {
            LeftRight::Left => (self as usize).wrapping_sub(num_turns) & 0b11,
            LeftRight::Right => (self as usize).wrapping_add(num_turns) & 0b11,
        })
    }
}

enum Action {
    North(usize),
    East(usize),
    South(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl Action {
    fn parse(input: &str) -> Option<Action> {
        let value = (&input[1..]).parse().ok()?;
        Some(match input.as_bytes()[0] {
            b'N' => Action::North(value),
            b'E' => Action::East(value),
            b'S' => Action::South(value),
            b'W' => Action::West(value),
            b'L' => Action::Left(value),
            b'R' => Action::Right(value),
            b'F' => Action::Forward(value),
            _ => return None,
        })
    }
}

#[aoc(day12, part1)]
fn solve_d12_p1(input: &str) -> usize {
    let mut x = 0isize;
    let mut y = 0isize;
    let mut ship_dir = Direction::East;
    for action in input.split('\n').map(|x| Action::parse(x).unwrap()) {
        match action {
            Action::North(distance) => y += distance as isize,
            Action::East(distance) => x += distance as isize,
            Action::South(distance) => y -= distance as isize,
            Action::West(distance) => x -= distance as isize,
            Action::Left(degrees) => ship_dir = ship_dir.rotate_ship(degrees, LeftRight::Left),
            Action::Right(degrees) => ship_dir = ship_dir.rotate_ship(degrees, LeftRight::Right),
            Action::Forward(distance) => {
                let (x_step, y_step) = ship_dir.xy();
                x += x_step * distance as isize;
                y += y_step * distance as isize;
            }
        }
    }
    x.abs() as usize + y.abs() as usize
}

#[derive(Debug, Copy, Clone)]
struct Waypoint {
    x: isize,
    y: isize,
}

impl Waypoint {
    fn rotate(self, mut degrees: usize, lr: LeftRight) -> Self {
        assert!(degrees == 90 || degrees == 180 || degrees == 270);
        if lr == LeftRight::Left {
            degrees = 360 - degrees;
        }
        match degrees {
            0 => self,
            90 => Waypoint {
                x: self.y,
                y: -self.x,
            },
            180 => Waypoint {
                x: -self.x,
                y: -self.y,
            },
            270 => Waypoint {
                x: -self.y,
                y: self.x,
            },
            _ => unreachable!(),
        }
    }
}

#[aoc(day12, part2)]
fn solve_d12_p2(input: &str) -> usize {
    let mut waypoint = Waypoint { x: 10, y: 1 };
    let mut x = 0isize;
    let mut y = 0isize;
    for action in input.split('\n').map(|x| Action::parse(x).unwrap()) {
        match action {
            Action::North(distance) => waypoint.y += distance as isize,
            Action::East(distance) => waypoint.x += distance as isize,
            Action::South(distance) => waypoint.y -= distance as isize,
            Action::West(distance) => waypoint.x -= distance as isize,
            Action::Left(degrees) => waypoint = waypoint.rotate(degrees, LeftRight::Left),
            Action::Right(degrees) => waypoint = waypoint.rotate(degrees, LeftRight::Right),
            Action::Forward(distance) => {
                x += waypoint.x * distance as isize;
                y += waypoint.y * distance as isize;
            }
        }
    }
    x.abs() as usize + y.abs() as usize
}
