use std::{ops::Index, time::Instant};

use bitvec::prelude::*;

use smallvec::SmallVec;

#[derive(Debug, Clone)]
struct Grid {
    data: BitVec,
    width: usize,
    height: usize,
}

impl Grid {
    fn calc_data_idx(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        let idx = self.calc_data_idx(x, y);
        self.data.set(idx, value);
    }
}

impl TryFrom<&str> for Grid {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let s: &str = value.as_ref();
        let Some(width) = s.lines().next().map(str::len) else {
            return Err(());
        };
        let Some(data): Option<BitVec> = s
            .lines()
            .flat_map(|line| {
                assert!(line.len() == width);
                line.chars().map(|c| match c {
                    '.' => Some(false),
                    '@' => Some(true),
                    _ => None,
                })
            })
            .collect()
        else {
            return Err(());
        };

        let height = data.len() / width;
        Ok(Self {
            data,
            width,
            height,
        })
    }
}

impl Index<(isize, isize)> for Grid {
    type Output = bool;

    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        assert!(x >= 0 && y >= 0);
        &self[(x as usize, y as usize)]
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[self.width * y + x]
    }
}

fn parse_input(input_data: String) -> Option<Grid> {
    Grid::try_from(input_data.as_ref()).ok()
}

fn iter_neighbours(
    x: isize,
    y: isize,
    width: isize,
    height: isize,
) -> impl Iterator<Item = (isize, isize)> {
    (-1..2).flat_map(move |x_off| {
        (-1..2).filter_map(move |y_off| {
            if x_off == 0 && y_off == 0 {
                return None;
            }

            let n_x = x + x_off;
            let n_y = y + y_off;

            if n_x < 0 || n_y < 0 || n_x >= width || n_y >= height {
                return None;
            }

            Some((n_x, n_y))
        })
    })
}

fn find_neighbouring_rolls(
    grid: &Grid,
    x: isize,
    y: isize,
) -> impl Iterator<Item = (isize, isize)> {
    iter_neighbours(x, y, grid.width as isize, grid.height as isize)
        .filter(|&(n_x, n_y)| grid[(n_x, n_y)])
}

fn can_roll_be_removed(grid: &Grid, x: isize, y: isize) -> bool {
    find_neighbouring_rolls(grid, x, y).count() < 4
}

fn find_moveable_rolls(grid: &Grid) -> impl Iterator<Item = (usize, usize)> {
    (0..grid.width).flat_map(move |x| {
        (0..grid.height).filter_map(move |y| {
            if !grid[(x, y)] {
                return None;
            }

            if can_roll_be_removed(grid, x as isize, y as isize) {
                Some((x, y))
            } else {
                None
            }
        })
    })
}

fn solve_task2_naive(mut grid: Grid) -> usize {
    let mut removed_rolls = 0;
    loop {
        let moveable_rolls = find_moveable_rolls(&grid).collect::<Vec<_>>();

        if moveable_rolls.is_empty() {
            break;
        }

        removed_rolls += moveable_rolls.len();

        for (x, y) in moveable_rolls {
            grid.set(x, y, false);
        }
    }

    removed_rolls
}

fn solve_task2_smarter(mut grid: Grid) -> usize {
    let grid_height = grid.height;
    let mut whole_iter =
        (0..grid.width as isize).flat_map(|x| (0..grid_height as isize).map(move |y| (x, y)));
    let mut stack = Vec::new();

    let mut removed_rolls = 0;

    loop {
        let (x, y) = match stack.pop() {
            Some((x, y)) => (x, y),
            None => match whole_iter.next() {
                Some((x, y)) => (x, y),
                None => break,
            },
        };

        if !grid[(x, y)] {
            continue;
        }

        let neighbouring_rolls =
            find_neighbouring_rolls(&grid, x, y).collect::<SmallVec<[(isize, isize); 8]>>();

        if neighbouring_rolls.len() < 4 {
            grid.set(x as usize, y as usize, false);
            removed_rolls += 1;
            stack.extend(neighbouring_rolls);
        }
    }
    removed_rolls
}

fn solve_task2_smartest(mut grid: Grid) -> usize {
    let grid_ = &grid;
    let mut num_neighbours = (0..grid.height as isize)
        .flat_map(|y| {
            (0..grid.width as isize).map(move |x| find_neighbouring_rolls(grid_, x, y).count())
        })
        .collect::<Vec<usize>>();

    let mut stack = num_neighbours
        .iter()
        .enumerate()
        .filter_map(|(pos, num_neighbours)| {
            if *num_neighbours < 4 {
                let y = pos / grid.width;
                let x = pos - grid.width * y;
                if grid[(x, y)] { Some((x, y)) } else { None }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut removed_rolls = 0;
    while let Some((x, y)) = stack.pop() {
        if !grid[(x, y)] {
            continue;
        }
        debug_assert!(grid[(x, y)]);
        debug_assert!(num_neighbours[grid.width * y + x] < 4);

        grid.set(x, y, false);
        removed_rolls += 1;

        for (n_x, n_y) in iter_neighbours(
            x as isize,
            y as isize,
            grid.width as isize,
            grid.height as isize,
        ) {
            if !grid[(n_x, n_y)] {
                continue;
            }

            debug_assert!(n_x >= 0 && n_y >= 0);

            let idx = grid.width * n_y as usize + n_x as usize;
            debug_assert!(num_neighbours[idx] > 0, "{x}, {y}, {n_x}, {n_y}, {idx}");
            num_neighbours[idx] -= 1;

            if num_neighbours[idx] < 4 {
                stack.push((n_x as usize, n_y as usize));
            }
        }
    }

    removed_rolls
}

pub fn solve_day04(input_data: String) {
    let input = match parse_input(input_data) {
        Some(input) => input,
        None => {
            println!("Could not parse input!");
            return;
        }
    };

    let start_1 = Instant::now();
    let sol_task1: usize = find_moveable_rolls(&input).count();
    let end_1 = Instant::now();
    println!(
        "Task 1:\t\t{sol_task1}\tTook: {}µs",
        (end_1 - start_1).as_micros()
    );

    let input_ = input.clone();
    let start_2_naive = Instant::now();
    let sol_task2_naive: usize = solve_task2_naive(input_);
    let end_2_naive = Instant::now();
    println!(
        "Naive 2:\t{sol_task2_naive}\tTook: {}µs",
        (end_2_naive - start_2_naive).as_micros()
    );

    let input_ = input.clone();
    let start_2_smarter = Instant::now();
    let sol_task2_smarter: usize = solve_task2_smarter(input_);
    let end_2_smarter = Instant::now();
    println!(
        "Smarter 2:\t{sol_task2_smarter}\tTook: {}µs",
        (end_2_smarter - start_2_smarter).as_micros()
    );

    let input_ = input.clone();
    let start_2_smartest = Instant::now();
    let sol_task2_smartest: usize = solve_task2_smartest(input_);
    let end_2_smartest = Instant::now();
    println!(
        "Smartest 2:\t{sol_task2_smartest}\tTook: {}µs",
        (end_2_smartest - start_2_smartest).as_micros()
    );
}
