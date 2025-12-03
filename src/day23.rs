use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Cup(u32);
impl Cup {
    fn as_idx(self) -> usize {
        self.0 as usize - 1
    }

    fn from_idx(idx: usize) -> Self {
        Cup(idx as u32 + 1)
    }
}

#[derive(Debug)]
struct Cups {
    current: Cup,
    cups: Box<[Cup]>,
}

impl Cups {
    fn parse(input: &[u8], total_len: usize) -> Option<Self> {
        let mut cups = vec![Cup(0); total_len];
        let input_iter = input
            .iter()
            .map(|x| (x - b'0') as u32)
            .chain(input.len() as u32 + 1..=total_len as u32);
        for (curr, next) in input_iter.clone().zip(input_iter.cycle().skip(1)) {
            cups[curr as usize - 1] = Cup(next as u32);
        }

        Some(Cups {
            current: Cup((input[0] - b'0') as u32),
            cups: cups.into_boxed_slice(),
        })
    }

    fn do_move(&mut self) {
        let taken_1 = self.next_cup(self.current);
        let taken_2 = self.next_cup(taken_1);
        let taken_3 = self.next_cup(taken_2);

        let mut dest_cup =
            Cup::from_idx((self.current.as_idx() + self.cups.len() - 1) % self.cups.len());
        Cup(((self.current.0 as usize - 1 + self.cups.len() - 1) % self.cups.len()) as u32 + 1);
        while dest_cup == taken_1 || dest_cup == taken_2 || dest_cup == taken_3 {
            dest_cup = Cup::from_idx((dest_cup.as_idx() + self.cups.len() - 1) % self.cups.len());
        }
        self.cups[self.current.as_idx()] = self.next_cup(taken_3);
        self.cups[taken_3.as_idx()] = self.next_cup(dest_cup);
        self.cups[dest_cup.as_idx()] = taken_1;
        self.current = self.next_cup(self.current);
    }

    fn next_cup(&self, cup: Cup) -> Cup {
        self.cups[cup.as_idx()]
    }

    fn iter(&self, starting_cup: Cup) -> impl Iterator<Item = Cup> + '_ {
        std::iter::successors(Some(starting_cup), move |&prev_cup| {
            Some(self.next_cup(prev_cup))
        })
    }
}

#[aoc(day23, part1)]
fn solve_d23_p1(input: &[u8]) -> usize {
    let mut cups = Cups::parse(input, input.len()).unwrap();
    for _ in 0..100 {
        cups.do_move();
    }
    cups.iter(Cup(1))
        .skip(1)
        .take_while(|&cup| cup != Cup(1))
        .fold(0, |accum, cup| accum * 10 + cup.0 as usize)
}

#[aoc(day23, part2)]
fn solve_d23_p2(input: &[u8]) -> usize {
    let mut cups = Cups::parse(input, 1_000_000).unwrap();
    for _ in 0..10_000_000 {
        cups.do_move();
    }
    cups.iter(Cup(1))
        .skip(1)
        .take(2)
        .map(|x| x.0 as usize)
        .product()
}
