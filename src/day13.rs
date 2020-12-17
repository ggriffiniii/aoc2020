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
