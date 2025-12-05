use std::time::Instant;

#[derive(Debug)]
struct Bank {
    batteries: Vec<u32>,
}

impl Bank {
    fn from_str<S: AsRef<str>>(s: S) -> Option<Self> {
        let s_ = s.as_ref();

        let batteries = s_
            .chars()
            .map(|c| c.to_digit(10))
            .collect::<Option<Vec<u32>>>()?;

        Some(Self { batteries })
    }
}

fn parse_input(input_data: String) -> Option<Vec<Bank>> {
    input_data.lines().map(Bank::from_str).collect()
}

fn max_jolts(bank: &[Bank], num_batteries: usize) -> impl Iterator<Item = u64> {
    bank.iter().map(move |bank| {
        let mut start_idx = 0;
        let mut acc: u64 = 0;

        for battery_idx in (0..num_batteries).rev() {
            let (max_idx, max_val) = bank.batteries
                [start_idx..(bank.batteries.len() - battery_idx)]
                .iter()
                .enumerate()
                .max_by(|&(idx_a, val_a), &(idx_b, val_b)| val_a.cmp(val_b).then(idx_b.cmp(&idx_a)))
                .expect("The bank has not enough batteries!");

            start_idx += max_idx + 1;
            acc = acc * 10 + *max_val as u64;
        }

        acc
    })
}

pub fn solve_day03(input_data: String) {
    let input = match parse_input(input_data) {
        Some(input) => input,
        None => {
            println!("Could not parse input!");
            return;
        }
    };

    let start_1 = Instant::now();
    let sol_task1: u64 = max_jolts(&input, 2).sum();
    let end_1 = Instant::now();
    println!(
        "Task 1: {sol_task1}\tTook: {}µs",
        (end_1 - start_1).as_micros()
    );

    let start_2 = Instant::now();
    let sol_task2: u64 = max_jolts(&input, 12).sum();
    let end_2 = Instant::now();
    println!(
        "Task 2: {sol_task2}\tTook: {}µs",
        (end_2 - start_2).as_micros()
    );
}
