use std::time::Instant;

use smallvec::{SmallVec, smallvec};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MathOp {
    Add,
    Mul,
}

impl TryFrom<&str> for MathOp {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            return Err(());
        }

        match value.chars().next().unwrap() {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Mul),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Problem {
    inputs: SmallVec<[u64; 4]>,
    op: Option<MathOp>,
}

impl Problem {
    pub fn solve(&self) -> u64 {
        match self.op.unwrap() {
            MathOp::Add => self.inputs.iter().sum(),
            MathOp::Mul => self.inputs.iter().product(),
        }
    }
}

fn parse_input1<S: AsRef<str>>(input_data_: S) -> Option<Vec<Problem>> {
    let input_data = input_data_.as_ref();
    let mut problems = Vec::<Problem>::new();

    for line in input_data.lines() {
        let mut splits = line.split_ascii_whitespace();
        let Some(first_elem_str) = splits.next() else {
            return None;
        };
        if let Ok(first_op) = first_elem_str.trim().try_into() {
            problems[0].op = Some(first_op);

            for (i, math_op_str) in splits.enumerate() {
                let Ok(op) = math_op_str.try_into() else {
                    return None;
                };
                problems[i + 1].op = Some(op);
            }
        } else {
            let Ok(first_num) = first_elem_str.parse() else {
                return None;
            };
            if problems.is_empty() {
                problems.push(Problem {
                    inputs: smallvec![first_num],
                    op: None,
                });
            } else {
                problems[0].inputs.push(first_num);
            }
            for (i, num_str) in splits.enumerate() {
                let Ok(num) = num_str.parse() else {
                    return None;
                };
                if problems.len() <= i + 1 {
                    problems.push(Problem {
                        inputs: smallvec![num],
                        op: None,
                    });
                } else {
                    problems[i + 1].inputs.push(num);
                }
            }
        }
    }

    Some(problems)
}

fn parse_input2<S: AsRef<str>>(input_data_: S) -> Option<Vec<Problem>> {
    let input_data = input_data_.as_ref();
    let mut transposed_input_data = Vec::new();

    let mut lines = input_data.lines().peekable();

    let mut problems = Vec::new();

    loop {
        let Some(line) = lines.next() else {
            break;
        };
        if lines.peek().is_some() {
            for (idx, c) in line.chars().enumerate() {
                if transposed_input_data.len() <= idx {
                    assert!(transposed_input_data.len() == idx);
                    transposed_input_data.push(String::from(""));
                }

                transposed_input_data[idx].push(c);
            }
        } else {
            for op_str in line.split_ascii_whitespace() {
                let Ok(op) = op_str.try_into() else {
                    return None;
                };
                problems.push(Problem {
                    inputs: smallvec!(),
                    op: Some(op),
                });
            }
        }
    }

    let mut idx = 0;
    for line in transposed_input_data {
        if line.trim() == "" {
            idx += 1;
            continue;
        }
        for split in line.split_ascii_whitespace() {
            let Ok(num) = split.parse() else {
                return None;
            };
            problems[idx].inputs.push(num);
        }
    }

    Some(problems)
}

fn solve_problems(problems: &[Problem]) -> impl Iterator<Item = u64> {
    problems.iter().map(Problem::solve)
}

pub fn solve_day06(input_data: String) {
    let input1 = match parse_input1(&input_data) {
        Some(input) => input,
        None => {
            eprintln!("Could not parse input1!");
            return;
        }
    };

    let input2 = match parse_input2(&input_data) {
        Some(input) => input,
        None => {
            eprintln!("Could not parse input2!");
            return;
        }
    };

    let start_1 = Instant::now();
    let sol_task1: u64 = solve_problems(&input1).sum();
    let end_1 = Instant::now();
    println!(
        "Task 1:\t\t{sol_task1}\tTook: {}µs",
        (end_1 - start_1).as_micros()
    );

    let start_2 = Instant::now();
    let sol_task2: u64 = solve_problems(&input2).sum();
    let end_2 = Instant::now();
    println!(
        "Task 2:\t\t{sol_task2}\tTook: {}µs",
        (end_2 - start_2).as_micros()
    );
}
