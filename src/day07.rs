use std::{ops::Not, time::Instant};

use bitvec::prelude::*;

#[derive(Debug, Clone)]
struct Grid {
    data: BitVec,
    width: usize,
    height: usize,
}

struct Manifold {
    start: usize,
    grid: Grid,
}

fn parse_input(input_data: String) -> Option<Manifold> {
    let mut lines = input_data.lines();
    let Some(first_line) = lines.next() else {
        return None;
    };

    let Some(start) = first_line.find("S") else {
        return None;
    };

    let width = first_line.len();
    let mut height = 1;
    let grid_data: BitVec = vec![false; width]
        .into_iter()
        .chain(lines.flat_map(|line| {
            height += 1;
            line.chars().map(|c| c == '^')
        }))
        .collect();

    let grid = Grid {
        data: grid_data,
        width,
        height,
    };
    Some(Manifold { start, grid })
}

fn num_beam_splits(manifold: &Manifold) -> u64 {
    let mut num_splits = 0;

    let mut state: BitVec<usize, Lsb0> = BitVec::repeat(false, manifold.grid.width);
    state.set(manifold.start, true);

    for y in 1..manifold.grid.height {
        let splitters: BitVec =
            manifold.grid.data[y * manifold.grid.width..(y + 1) * manifold.grid.width].into();

        let mut new_state = BitVec::repeat(false, manifold.grid.width);
        for x in 0..manifold.grid.width {
            if state[x] {
                if splitters[x] {
                    num_splits += 1;
                    if let Some(l_x) = x.checked_sub(1) {
                        new_state.set(l_x, true);
                    }

                    if x + 1 < manifold.grid.width {
                        new_state.set(x + 1, true);
                    }
                } else {
                    new_state.set(x, true);
                }
            }
        }
        state = new_state & splitters.not();
    }

    num_splits
}

fn num_timelines(manifold: &Manifold) -> u64 {
    let mut state = vec![0; manifold.grid.width];
    state[manifold.start] = 1;

    for y in 1..manifold.grid.height {
        let splitters: BitVec =
            manifold.grid.data[y * manifold.grid.width..(y + 1) * manifold.grid.width].into();

        let mut new_state = vec![0; manifold.grid.width];
        for x in 0..manifold.grid.width {
            if state[x] > 0 {
                if splitters[x] {
                    if let Some(l_x) = x.checked_sub(1) {
                        new_state[l_x] += state[x];
                    }

                    if x + 1 < manifold.grid.width {
                        new_state[x + 1] += state[x];
                    }
                } else {
                    new_state[x] += state[x];
                }
            }
        }
        for (idx, new_state_entry) in new_state.iter_mut().enumerate() {
            if splitters[idx] {
                *new_state_entry = 0;
            }
        }
        state = new_state;
    }

    state.into_iter().sum()
}

pub fn solve_day07(input_data: String) {
    let input = match parse_input(input_data) {
        Some(input) => input,
        None => {
            eprintln!("Could not parse input!");
            return;
        }
    };

    let start_1 = Instant::now();
    let sol_task1 = num_beam_splits(&input);
    let end_1 = Instant::now();
    println!(
        "Task 1:\t{sol_task1}\tTook: {}µs",
        (end_1 - start_1).as_micros()
    );

    let start_2 = Instant::now();
    let sol_task2 = num_timelines(&input);
    let end_2 = Instant::now();
    println!(
        "Task 2:\t{sol_task2}\tTook: {}µs",
        (end_2 - start_2).as_micros()
    );
}
