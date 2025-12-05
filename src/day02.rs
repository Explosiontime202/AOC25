use std::time::Instant;

type ID = u64;
#[derive(Debug)]
struct IDRange {
    start: ID,
    end: ID,
}

impl IDRange {
    fn from_str<S: AsRef<str>>(s: S) -> Option<Self> {
        let s_ = s.as_ref();
        let mut ids = s_.split("-");
        let Some(start_str) = ids.next() else {
            return None;
        };
        let Some(end_str) = ids.next() else {
            return None;
        };
        if ids.next().is_some() {
            return None;
        }
        let start: ID = match start_str.parse() {
            Ok(start) => start,
            Err(_) => return None,
        };
        let end: ID = match end_str.parse() {
            Ok(end) => end,
            Err(_) => return None,
        };

        Some(Self { start, end })
    }
}

fn find_invalid_ids_task_1(input: &[IDRange]) -> impl Iterator<Item = u64> {
    input
        .iter()
        .flat_map(|range| range.start..=range.end)
        .filter(|id| {
            let num_id_digits = id.ilog10() + 1;

            // ignore numbers where we would need to consider leading zeros
            if num_id_digits % 2 != 0 {
                return false;
            }

            let digit_int = 10u64.pow(num_id_digits / 2);

            let first_half = id / digit_int;
            let second_half = id % digit_int;

            first_half == second_half
        })
}

fn find_invalid_ids_task_1_slow(input: &[IDRange]) -> impl Iterator<Item = u64> {
    input
        .iter()
        .flat_map(|range| range.start..=range.end)
        .filter(|id| {
            let id_str = id.to_string();
            let num_id_digits = id_str.len();

            // ignore numbers where we would need to consider leading zeros
            if num_id_digits % 2 != 0 {
                return false;
            }

            id_str[0..(num_id_digits / 2)] == id_str[(num_id_digits / 2)..]
        })
}

fn find_invalid_ids_task2(input: &[IDRange]) -> impl Iterator<Item = u64> {
    input
        .iter()
        .flat_map(|range| range.start..=range.end)
        .filter(|id| {
            let num_id_digits = id.ilog10() + 1;

            for num_pieces in 2..=num_id_digits {
                if !num_id_digits.is_multiple_of(num_pieces) {
                    continue;
                }

                let digit_int = 10u64.pow(num_id_digits / num_pieces);
                let mut running_digit_int = digit_int;

                let first_piece = id % digit_int;
                let mut is_valid = false;
                for _ in 0..(num_pieces - 1) {
                    let piece = (id / running_digit_int) % digit_int;
                    if piece != first_piece {
                        is_valid = true;
                        break;
                    }
                    running_digit_int *= digit_int;
                }

                if !is_valid {
                    return true;
                }
            }

            false
        })
}

fn find_invalid_ids_task2_slow(input: &[IDRange]) -> impl Iterator<Item = u64> {
    input
        .iter()
        .flat_map(|range| range.start..=range.end)
        .filter(|id| {
            let id_str = id.to_string();
            let num_id_digits = id_str.len();

            for num_pieces in 2..=num_id_digits {
                if !num_id_digits.is_multiple_of(num_pieces) {
                    continue;
                }

                let digits_per_piece = num_id_digits / num_pieces;
                let first_piece = &id_str[0..digits_per_piece];
                let mut is_valid = false;
                for piece_idx in 1..num_pieces {
                    let piece = &id_str
                        [(digits_per_piece * piece_idx)..(digits_per_piece * (piece_idx + 1))];
                    if piece != first_piece {
                        is_valid = true;
                        break;
                    }
                }

                if !is_valid {
                    return true;
                }
            }

            false
        })
}

fn parse_input(input_data: String) -> Option<Vec<IDRange>> {
    let ranges_opt_it = input_data.trim().split(",").map(IDRange::from_str);

    let mut ranges = Vec::new();
    for range_opt in ranges_opt_it {
        let Some(range) = range_opt else {
            return None;
        };
        ranges.push(range);
    }
    Some(ranges)
}

pub fn solve_day02(input_data: String) {
    let input = match parse_input(input_data) {
        Some(input) => input,
        None => {
            println!("Could not parse input!");
            return;
        }
    };

    let start_1 = Instant::now();
    let sol_task1: u64 = find_invalid_ids_task_1(&input).sum();
    let end_1 = Instant::now();
    println!(
        "Task 1: {sol_task1}\tTook: {}µs",
        (end_1 - start_1).as_micros()
    );

    let start_1_slow = Instant::now();
    let sol_task1_slow: u64 = find_invalid_ids_task_1_slow(&input).sum();
    let end_1_slow = Instant::now();
    println!(
        "Slow 1: {sol_task1_slow}\tTook: {}µs",
        (end_1_slow - start_1_slow).as_micros()
    );

    let start_2 = Instant::now();
    let sol_task2: u64 = find_invalid_ids_task2(&input).sum();
    let end_2: Instant = Instant::now();
    println!(
        "Task 2: {sol_task2}\tTook: {}µs",
        (end_2 - start_2).as_micros()
    );

    let start_2_slow = Instant::now();
    let sol_task2_slow: u64 = find_invalid_ids_task2_slow(&input).sum();
    let end_2_slow: Instant = Instant::now();
    println!(
        "Slow 2: {sol_task2_slow}\tTook: {}µs",
        (end_2_slow - start_2_slow).as_micros()
    );
}
