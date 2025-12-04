use aoc_runner_derive::aoc;

#[aoc(day13, part1)]
fn solve_d13_p1(input: &str) -> usize {
    let first_newline = input.find('\n').unwrap();
    let (t, bus_ids) = input.split_at(first_newline);
    let bus_ids = bus_ids.trim();

    let t: usize = t.parse().unwrap();

    struct WaitTime {
        bus_id: usize,
        wait_time: usize,
    }
    let min_wait_time = bus_ids
        .split(',')
        .filter(|&x| x != "x")
        .map(|x| x.parse::<usize>().unwrap())
        .fold(
            WaitTime {
                bus_id: usize::MAX,
                wait_time: usize::MAX,
            },
            |min, bus_id| {
                let offset = t % bus_id;
                let wait_time = if offset == 0 { 0 } else { bus_id - offset };
                if min.wait_time < wait_time {
                    min
                } else {
                    WaitTime { bus_id, wait_time }
                }
            },
        );
    min_wait_time.bus_id * min_wait_time.wait_time
}

// Compute the modular inverse t such that a*t ≡ 1 mod n
// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Modular_integers
fn inverse(a: isize, n: isize) -> isize {
    let mut t = 0;
    let mut r = n;
    let mut newt = 1;
    let mut newr = a;

    while newr != 0 {
        let q = r / newr;
        let prev_t = t;
        t = newt;
        newt = prev_t - q * newt;
        let prev_r = r;
        r = newr;
        newr = prev_r - q * newr;
    }

    if r > 1 {
        panic!("{a} is not invertible");
    }
    if t < 0 {
        t += n;
    }
    t
}

// All the bus numbers are prime numbers. Use the chinese remainder theorem:
// https://www.youtube.com/watch?v=MdePzlQtnCc
#[aoc(day13, part2)]
fn solve_d13_p2(input: &str) -> usize {
    let (_, bus_ids) = input.split_once('\n').unwrap();
    let mut ri = Vec::new();
    let mut mods: Vec<usize> = Vec::new();
    let mut unique_mod = 0;
    for (time_after, bus_id) in bus_ids.split(',').enumerate() {
        if bus_id == "x" {
            continue;
        }
        let bus_id: usize = bus_id.parse().unwrap();
        mods.push(bus_id);
        ri.push((bus_id - time_after % bus_id) % bus_id);
    }

    let unique_mod: usize = mods.iter().product();
    let mi: Vec<_> = mods.iter().map(|m| unique_mod / m).collect();
    let xi: Vec<_> = mods
        .iter()
        .zip(mi.iter())
        .map(|(&m, &mi): (&usize, &usize)| inverse(mi as isize, m as isize) as usize)
        .collect();

    ri.iter()
        .copied()
        .zip(mi.iter().copied().zip(xi.iter().copied()))
        .map(|(ri, (mi, xi))| ri * mi * xi)
        .sum::<usize>()
        % unique_mod
}
