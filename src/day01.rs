use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    #[allow(unused)]
    fn to_char(&self) -> char {
        match self {
            Self::Left => 'L',
            Self::Right => 'R',
        }
    }

    fn to_factor(&self) -> i64 {
        match self {
            Self::Left => -1,
            Self::Right => 1,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Rotation {
    dir: Direction,
    distance: u64,
}

fn parse_input(input_data: String) -> Option<Vec<Rotation>> {
    input_data
        .lines()
        .map(|line| {
            if line.len() >= 2
                && let Ok(dir) = line.chars().next().unwrap().try_into()
                && let Ok(distance) = line[1..].parse()
            {
                Some(Rotation { dir, distance })
            } else {
                None
            }
        })
        .collect()
}

fn count_zero_wheel(input: &[Rotation]) -> u64 {
    let mut wheel_zeros = 0;
    let mut wheel = 50i64;
    for instruction in input {
        wheel += (instruction.distance as i64) * instruction.dir.to_factor();

        wheel = ((wheel % 100) + 100) % 100;

        if wheel == 0 {
            wheel_zeros += 1;
        }
    }

    wheel_zeros
}

fn count_zero_wheel_with_intermediate(input: &[Rotation]) -> u64 {
    let mut wheel_zeros = 0;
    let mut wheel = 50i64;
    for instruction in input {
        assert!(wheel >= 0);
        let new_wheel = wheel + (instruction.distance as i64) * instruction.dir.to_factor();

        let new_zeros = ((new_wheel - 50).abs() + 50) / 100;
        wheel_zeros += new_zeros as u64;

        let new_wheel = ((new_wheel % 100) + 100) % 100;

        if instruction.dir == Direction::Left && wheel == 0 {
            // avoid double counting zeros at the end
            wheel_zeros -= 1;
        }
        wheel = new_wheel;
    }

    wheel_zeros
}

fn simulate_wheel(input: &[Rotation]) -> u64 {
    let mut wheel_zeros = 0;
    let mut wheel = 50i64;
    for instruction in input {
        for _ in 0..instruction.distance {
            assert!(wheel >= 0);
            wheel += instruction.dir.to_factor();

            wheel = ((wheel % 100) + 100) % 100;
            if wheel == 0 {
                wheel_zeros += 1;
            }
        }
    }
    wheel_zeros
}

pub fn solve_day01(input_data: String) {
    let input = match parse_input(input_data) {
        Some(input) => input,
        None => {
            eprintln!("Could not parse input!");
            return;
        }
    };

    let start_1 = Instant::now();
    let sol_task1 = count_zero_wheel(&input);
    let end_1 = Instant::now();
    println!(
        "Task1:\t{}\tTook: {}µs",
        sol_task1,
        (end_1 - start_1).as_micros()
    );
    let start_2_good = Instant::now();
    let sol_task2 = count_zero_wheel_with_intermediate(&input);
    let end_2_good = Instant::now();
    println!(
        "Task2:\t{}\tTook: {}µs",
        sol_task2,
        (end_2_good - start_2_good).as_micros()
    );
    let start_2_bad = Instant::now();
    let sol_task2_bad = simulate_wheel(&input);
    let end_2_bad = Instant::now();
    println!(
        "Sim2:\t{}\tTook: {}µs",
        sol_task2_bad,
        (end_2_bad - start_2_bad).as_micros()
    );
}
