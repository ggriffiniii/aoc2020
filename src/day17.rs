use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day17, part1)]
fn solve_d17_p1(input: &str) -> usize {
    let mut world = HashSet::new();
    for (y, line) in input.split('\n').enumerate() {
        for (x, _) in line.as_bytes().iter().copied().enumerate().filter(|&(_, byte)| {
            byte == b'#'
        }) {
            world.insert((x as isize, y as isize, 0isize, 0isize));
        }
    }
    for _ in 0 .. 6 {
        world = step(world);
    }
    world.len()
}

fn neighbors(world: &HashSet<(isize, isize, isize, isize)>, (x, y, z, w): (isize, isize, isize, isize)) -> usize {
    let mut count = 0;
    for xa in x-1..=x+1 {
        for ya in y-1..=y+1 {
            for za in z-1..=z+1 {
                for wa in w-1..=w+1 {
                    count += world.contains(&(xa, ya, za, wa)) as usize;
                }
            }
        }
    }
    count - world.contains(&(x, y, z, w)) as usize
}

fn step(world: HashSet<(isize, isize, isize, isize)>) -> HashSet<(isize, isize, isize, isize)> {
    struct MinMax {
        x_min: isize,
        x_max: isize,
        y_min: isize,
        y_max: isize,
        z_min: isize,
        z_max: isize,
        w_min: isize,
        w_max: isize,
    }
    let MinMax{
        x_min, x_max, y_min, y_max, z_min, z_max, w_min, w_max,
    } = world.iter().fold(MinMax{
        x_min: isize::MAX,
        x_max: isize::MIN,
        y_min: isize::MAX,
        y_max: isize::MIN,
        z_min: isize::MAX,
        z_max: isize::MIN,
        w_min: isize::MAX,
        w_max: isize::MIN,
    }, |minmax, &(x, y, z, w)| {
        MinMax{
            x_min: std::cmp::min(minmax.x_min, x),
            x_max: std::cmp::max(minmax.x_max, x),
            y_min: std::cmp::min(minmax.y_min, y),
            y_max: std::cmp::max(minmax.y_max, y),
            z_min: std::cmp::min(minmax.z_min, z),
            z_max: std::cmp::max(minmax.z_max, z),
            w_min: std::cmp::min(minmax.w_min, w),
            w_max: std::cmp::max(minmax.w_max, w),
        }
    });
    let mut new_world = HashSet::new();
    for x in x_min -1 ..= x_max + 1 {
        for  y in y_min -1 ..= y_max + 1 {
            for z in z_min -1 ..= z_max + 1 {
                for w in w_min -1 ..= w_max + 1 {
                    let active = world.contains(&(x, y, z, w));
                    let n = neighbors(&world, (x, y, z, w));
                    if active && (n == 2 || n == 3) {
                        new_world.insert((x, y, z, w));
                    }
                    if !active && n == 3 {
                        new_world.insert((x, y, z, w));
                    }
                }
            }
        }
    }
    new_world
}


