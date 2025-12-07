use std::{cmp::Ordering, time::Instant};

type ID = u64;

#[derive(Debug, Clone, Copy)]
struct IDRange {
    start: ID,
    end: ID,
}

#[derive(Debug, Clone)]
struct DB {
    fresh_r: Vec<IDRange>,
    ids: Vec<ID>,
}

fn parse_input(input_data: String) -> Option<DB> {
    let mut splits = input_data.split("\n\n");
    let Some(ranges_str) = splits.next() else {
        return None;
    };
    let Some(ids_str) = splits.next() else {
        return None;
    };
    let None = splits.next() else {
        return None;
    };

    let ranges_opt: Option<Vec<IDRange>> = ranges_str
        .lines()
        .map(|line| {
            let mut range_splits = line.split("-");
            let Some(start_str) = range_splits.next() else {
                return None;
            };
            let Some(end_str) = range_splits.next() else {
                return None;
            };
            let None = range_splits.next() else {
                return None;
            };

            let Ok(start): Result<ID, _> = start_str.parse() else {
                return None;
            };
            let Ok(end): Result<ID, _> = end_str.parse() else {
                return None;
            };

            if start > end {
                return None;
            }

            Some(IDRange { start, end })
        })
        .collect();
    let Some(ranges) = ranges_opt else {
        return None;
    };

    let ids_opt: Option<Vec<ID>> = ids_str
        .lines()
        .map(|line| {
            let Ok(id) = line.parse() else {
                return None;
            };
            Some(id)
        })
        .collect();
    let Some(ids) = ids_opt else {
        return None;
    };

    Some(DB {
        fresh_r: ranges,
        ids,
    })
}

fn find_fresh_ids_naive(db: &DB) -> impl Iterator<Item = ID> {
    db.ids
        .iter()
        .filter(|&&id| {
            db.fresh_r
                .iter()
                .filter(|range| range.start <= id && range.end >= id)
                .next()
                .is_some()
        })
        .copied()
}

fn prepare_sorted_range_array(db: &mut DB) {
    if db.fresh_r.is_empty() {
        return;
    }
    db.fresh_r.sort_unstable_by_key(|range| range.start);
    let mut merged = Vec::with_capacity(db.fresh_r.len());
    // print!("merging: {} -> ", db.fresh_r.len());
    let mut current = db.fresh_r[0];
    for range in std::mem::take(&mut db.fresh_r) {
        if range.start <= current.end {
            if range.end > current.end {
                current.end = range.end
            }
        } else {
            merged.push(current);
            current = range;
        }
    }

    // also push the last one too
    merged.push(current);
    db.fresh_r = merged;
    println!("{}", db.fresh_r.len());
}

fn find_fresh_ids_smart(db: &mut DB) -> impl Iterator<Item = ID> {
    prepare_sorted_range_array(db);

    db.ids
        .iter()
        .filter(|&&id| {
            db.fresh_r
                .binary_search_by(|range| {
                    if range.start > id {
                        return Ordering::Greater;
                    }
                    if range.end < id {
                        return Ordering::Less;
                    }
                    Ordering::Equal
                })
                .is_ok()
        })
        .copied()
}

fn find_fresh_id_ranges(db: &mut DB) -> impl Iterator<Item = ID> {
    prepare_sorted_range_array(db);
    db.fresh_r.iter().flat_map(|range| range.start..=range.end)
}

pub fn solve_day05(input_data: String) {
    let input = match parse_input(input_data) {
        Some(input) => input,
        None => {
            eprintln!("Could not parse input!");
            return;
        }
    };

    let start_1_naive = Instant::now();
    let sol_task1_naive: usize = find_fresh_ids_naive(&input).count();
    let end_1_naive = Instant::now();
    println!(
        "Naive 1:\t\t{sol_task1_naive}\tTook: {}µs",
        (end_1_naive - start_1_naive).as_micros()
    );

    let mut input_ = input.clone();
    let start_1_smart = Instant::now();
    let sol_task1_smart: usize = find_fresh_ids_smart(&mut input_).count();
    let end_1_smart = Instant::now();
    println!(
        "Smart 1:\t\t{sol_task1_smart}\tTook: {}µs",
        (end_1_smart - start_1_smart).as_micros()
    );

    let mut input_ = input.clone();
    let start_2 = Instant::now();
    let sol_task2: usize = find_fresh_id_ranges(&mut input_).count();
    let end_2 = Instant::now();
    println!(
        "Task 2:\t\t{sol_task2}\tTook: {}µs",
        (end_2 - start_2).as_micros()
    );
}
