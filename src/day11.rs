use std::{collections::HashMap, time::Instant};

struct Input<'a> {
    devices: Vec<Vec<usize>>,
    device_map: HashMap<&'a str, usize>,
}

#[must_use]
fn parse_input<'a>(input_data: &'a str) -> Option<Input<'a>> {
    let mut devices = Vec::new();
    let mut device_map = HashMap::new();

    for line in input_data.lines() {
        let mut splits = line.split(": ");
        let Some(device_name) = splits.next() else {
            return None;
        };
        let Some(attached_str) = splits.next() else {
            return None;
        };
        if splits.next().is_some() {
            return None;
        }

        if !device_map.contains_key(device_name) {
            device_map.insert(device_name, devices.len());
            devices.push(Vec::new());
        }

        let device_idx = device_map[device_name];

        let attached = attached_str
            .split_ascii_whitespace()
            .map(|device| {
                let attached_idx = *device_map.entry(device).or_insert_with(|| {
                    let attached_idx = devices.len();
                    devices.push(Vec::new());

                    attached_idx
                });
                attached_idx
            })
            .collect::<Vec<_>>();

        let device = &mut devices[device_idx];
        assert!(device.is_empty());
        *device = attached;
    }

    Some(Input {
        devices,
        device_map,
    })
}

fn find_paths_naive_rec(start_idx: usize, end_idx: usize, input: &Input) -> usize {
    if start_idx == end_idx {
        return 1;
    }
    input.devices[start_idx]
        .iter()
        .map(|&next_idx| find_paths_naive_rec(next_idx, end_idx, input))
        .sum()
}

fn find_paths_naive(start: &str, end: &str, input: &Input) -> usize {
    let start_idx = input.device_map.get(start).expect("Not found start");
    let end_idx = input.device_map.get(end).expect("Not found end");
    find_paths_naive_rec(*start_idx, *end_idx, input)
}

fn find_paths_rec(
    start_idx: usize,
    end_idx: usize,
    input: &Input,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    if start_idx == end_idx {
        return 1;
    }
    if let Some(paths) = memo.get(&start_idx) {
        return *paths;
    }

    let num_paths = input.devices[start_idx]
        .iter()
        .map(|&next_idx| find_paths_rec(next_idx, end_idx, input, memo))
        .sum();

    memo.insert(start_idx, num_paths);
    num_paths
}

fn find_paths_memo(start_idx: usize, end_idx: usize, input: &Input) -> usize {
    let mut memo = HashMap::new();
    find_paths_rec(start_idx, end_idx, input, &mut memo)
}

fn find_paths_with_stop(
    start: &str,
    stop_a: &str,
    stop_b: &str,
    end: &str,
    input: &Input,
) -> usize {
    let start_idx = input.device_map.get(start).expect("Not found start");
    let stop_a_idx = input.device_map.get(stop_a).expect("Not found stop_a");
    let stop_b_idx = input.device_map.get(stop_b).expect("Not found stop_b");
    let end_idx = input.device_map.get(end).expect("Not found end");

    let num_paths_a = find_paths_memo(*start_idx, *stop_a_idx, input)
        * find_paths_memo(*stop_a_idx, *stop_b_idx, input)
        * find_paths_memo(*stop_b_idx, *end_idx, input);
    let num_paths_b = find_paths_memo(*start_idx, *stop_b_idx, input)
        * find_paths_memo(*stop_b_idx, *stop_a_idx, input)
        * find_paths_memo(*stop_a_idx, *end_idx, input);

    num_paths_a + num_paths_b
}

pub fn solve_day11(input_data: String) {
    let input = match parse_input(&input_data) {
        Some(input) => input,
        None => {
            eprintln!("Could not parse input!");
            return;
        }
    };

    let start_1 = Instant::now();
    let sol_task1 = find_paths_naive("you", "out", &input);
    let end_1 = Instant::now();
    println!(
        "Task 1: {sol_task1}\tTook: {}µs",
        (end_1 - start_1).as_micros()
    );

    let start_2 = Instant::now();
    let sol_task2 = find_paths_with_stop("svr", "dac", "fft", "out", &input);
    let end_2 = Instant::now();
    println!(
        "Task 2: {sol_task2}\tTook: {}µs",
        (end_2 - start_2).as_micros()
    );
}
