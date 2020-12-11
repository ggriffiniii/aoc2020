use aoc_runner_derive::aoc;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum GridSquare {
    Empty = b'L',
    Occupied = b'#',
    Floor = b'.',
}

impl GridSquare {
    fn from_u8(b: u8) -> Option<Self> {
        Some(match b {
            b'L' => GridSquare::Empty,
            b'#' => GridSquare::Occupied,
            b'.' => GridSquare::Floor,
            _ => return None,
        })
    }
    fn into_u8(self) -> u8 {
        match self {
            GridSquare::Empty => b'L',
            GridSquare::Occupied => b'#',
            GridSquare::Floor => b'.',
        }
    }
}

impl fmt::Display for GridSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.into_u8() as char)
    }
}

#[derive(Debug, Copy, Clone)]
struct GridIdx(usize);

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    row_len: usize,
    grid: Vec<GridSquare>,
}

impl Grid {
    fn parse(input: &str) -> Option<Self> {
        let row_len = input.find('\n')?;
        let grid = input
            .bytes()
            .filter(|&b| b != b'\n')
            .map(|b| GridSquare::from_u8(b))
            .collect::<Option<Vec<_>>>()?;
        Some(Grid { row_len, grid })
    }

    fn new(grid: Vec<GridSquare>, row_len: usize) -> Grid {
        if grid.len() % row_len != 0 {
            panic!("invalid grid");
        }
        Grid { grid, row_len }
    }

    fn enumerate(&self) -> impl Iterator<Item = (GridIdx, GridSquare)> + '_ {
        self.grid
            .iter()
            .copied()
            .enumerate()
            .map(|(idx, square)| (GridIdx(idx), square))
    }

    fn is_occupied(&self, idx: usize) -> bool {
        self.grid[idx] == GridSquare::Occupied
    }

    fn num_adjacent_occupied(&self, idx: GridIdx) -> usize {
        let mut total = 0;
        let col = idx.0 % self.row_len;

        // Add row above idx if it exists.
        if self.row_len < idx.0 {
            let above_idx = idx.0 - self.row_len;
            if col > 0 && self.is_occupied(above_idx - 1) {
                total += 1;
            }
            if self.is_occupied(above_idx) {
                total += 1;
            }
            if col < self.row_len - 1 && self.is_occupied(above_idx + 1) {
                total += 1;
            }
        }

        // Add left and right if they exist.
        if col > 0 && self.is_occupied(idx.0 - 1) {
            total += 1;
        }
        if col < self.row_len - 1 && self.is_occupied(idx.0 + 1) {
            total += 1;
        }

        // Add row below if it exists.
        let below_idx = idx.0 + self.row_len;
        if below_idx < self.grid.len() {
            if col > 0 && self.is_occupied(below_idx - 1) {
                total += 1;
            }
            if self.is_occupied(below_idx) {
                total += 1;
            }
            if col < self.row_len - 1 && self.is_occupied(below_idx + 1) {
                total += 1;
            }
        }
        total
    }

    fn num_visible_occupied(&self, idx: GridIdx) -> usize {
        let mut total = 0;
        for &(x_step, y_step) in &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            total += GridIter::new(self, idx, x_step, y_step)
                .find_map(|idx| match &self.grid[idx.0] {
                    GridSquare::Empty => Some(0),
                    GridSquare::Occupied => Some(1),
                    GridSquare::Floor => None,
                })
                .unwrap_or(0);
        }
        total
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.grid.chunks_exact(self.row_len) {
            for square in line {
                write!(f, "{}", square)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[aoc(day11, part1)]
fn solve_d11_p1(input: &str) -> usize {
    fn next_grid(orig: &Grid) -> Grid {
        let new_grid: Vec<GridSquare> = orig
            .enumerate()
            .map(|(grid_idx, grid_square)| match grid_square {
                GridSquare::Empty => {
                    let occupied = orig.num_adjacent_occupied(grid_idx);
                    if occupied == 0 {
                        GridSquare::Occupied
                    } else {
                        GridSquare::Empty
                    }
                }
                GridSquare::Occupied => {
                    let occupied = orig.num_adjacent_occupied(grid_idx);
                    if occupied >= 4 {
                        GridSquare::Empty
                    } else {
                        GridSquare::Occupied
                    }
                }
                GridSquare::Floor => GridSquare::Floor,
            })
            .collect();
        Grid::new(new_grid, orig.row_len)
    }
    let mut grid = Grid::parse(input).unwrap();
    let stable_grid = loop {
        let next = next_grid(&grid);
        if next == grid {
            break next;
        }
        grid = next;
    };
    stable_grid
        .enumerate()
        .filter(|&(_idx, square)| square == GridSquare::Occupied)
        .count()
}

#[derive(Debug)]
struct GridIter {
    x: isize,
    y: isize,
    x_step: isize,
    y_step: isize,
    num_cols: usize,
    num_rows: usize,
}
impl GridIter {
    fn new(grid: &Grid, idx: GridIdx, x_step: isize, y_step: isize) -> Self {
        let num_cols = grid.row_len;
        let num_rows = grid.grid.len() / num_cols;
        let x = (idx.0 % num_cols) as isize;
        let y = (idx.0 / num_cols) as isize;
        GridIter {
            x,
            y,
            x_step,
            y_step,
            num_cols,
            num_rows,
        }
    }
}

impl Iterator for GridIter {
    type Item = GridIdx;

    fn next(&mut self) -> Option<GridIdx> {
        self.x += self.x_step;
        self.y += self.y_step;
        if self.x < 0 || self.x >= self.num_cols as isize {
            return None;
        }
        if self.y < 0 || self.y >= self.num_rows as isize {
            return None;
        }
        Some(GridIdx(self.y as usize * self.num_cols + self.x as usize))
    }
}

#[aoc(day11, part2)]
fn solve_d11_p2(input: &str) -> usize {
    fn next_grid(orig: &Grid) -> Grid {
        let new_grid: Vec<GridSquare> = orig
            .enumerate()
            .map(|(grid_idx, grid_square)| match grid_square {
                GridSquare::Empty => {
                    let occupied = orig.num_visible_occupied(grid_idx);
                    if occupied == 0 {
                        GridSquare::Occupied
                    } else {
                        GridSquare::Empty
                    }
                }
                GridSquare::Occupied => {
                    let occupied = orig.num_visible_occupied(grid_idx);
                    if occupied >= 5 {
                        GridSquare::Empty
                    } else {
                        GridSquare::Occupied
                    }
                }
                GridSquare::Floor => GridSquare::Floor,
            })
            .collect();
        Grid::new(new_grid, orig.row_len)
    }
    let mut grid = Grid::parse(input).unwrap();
    let stable_grid = loop {
        let next = next_grid(&grid);
        if next == grid {
            break next;
        }
        grid = next;
    };
    stable_grid
        .enumerate()
        .filter(|&(_idx, square)| square == GridSquare::Occupied)
        .count()
}
