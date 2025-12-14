use std::{fmt::Display, iter, time::Instant};

use bitvec::prelude::*;
use z3::{Optimize, SatResult, ast::Int};

struct Machine {
    indicator_lights: BitVec,
    button_wiring: Vec<BitVec>,
    joltage_req: Vec<u16>,
}

impl TryFrom<&str> for Machine {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some(indicator_light_start) = value.find("[") else {
            return Err(());
        };
        let Some(indicator_light_end) = value.find("]") else {
            return Err(());
        };
        let Some(wiring_start) = value.find("(") else {
            return Err(());
        };
        let Some(wiring_end) = value.rfind(")") else {
            return Err(());
        };
        let Some(joltage_req_start) = value.find("{") else {
            return Err(());
        };
        let Some(joltage_req_end) = value.find("}") else {
            return Err(());
        };
        if joltage_req_end + 1 != value.len() {
            return Err(());
        }

        let mut indicator_lights: BitVec<usize, Lsb0> = BitVec::new();
        for c in value[indicator_light_start + 1..indicator_light_end].chars() {
            let v = match c {
                '.' => false,
                '#' => true,
                _ => return Err(()),
            };
            indicator_lights.push(v);
        }

        let Some(button_wiring) = value[(wiring_start + 1)..wiring_end]
            .split(") (")
            .map(|button| {
                let Some(toggled_lights) = button
                    .split(",")
                    .map(|idx_str| idx_str.parse())
                    .map(Result::ok)
                    .collect::<Option<Vec<usize>>>()
                else {
                    return None;
                };
                let mut wiring: BitVec<usize, Lsb0> = BitVec::repeat(false, indicator_lights.len());
                for light_idx in toggled_lights {
                    wiring.set(light_idx, true);
                }
                Some(wiring)
            })
            .collect()
        else {
            return Err(());
        };

        let Some(joltage_req) = value[(joltage_req_start + 1)..joltage_req_end]
            .split(",")
            .map(|num_str| num_str.parse())
            .map(Result::ok)
            .collect()
        else {
            return Err(());
        };

        Ok(Machine {
            indicator_lights,
            button_wiring,
            joltage_req,
        })
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {:?}, {:?}",
            self.indicator_lights, self.button_wiring, self.joltage_req
        )
    }
}

#[must_use]
fn parse_input(input_data: String) -> Option<Vec<Machine>> {
    input_data
        .lines()
        .map(Machine::try_from)
        .map(Result::ok)
        .collect()
}

fn min_button_presses_light(machines: &[Machine]) -> u32 {
    machines
        .iter()
        .map(|machine| {
            (0..(1u64 << machine.button_wiring.len()))
                .filter_map(|mut button_pattern| {
                    let num_buttons = button_pattern.count_ones();
                    let mut pattern: BitVec<usize, Lsb0> =
                        BitVec::repeat(false, machine.indicator_lights.len());

                    let mut idx = 0;
                    while button_pattern != 0 {
                        if button_pattern & 1 == 1 {
                            pattern ^= &machine.button_wiring[idx];
                        }
                        button_pattern >>= 1;
                        idx += 1;
                    }

                    if pattern == machine.indicator_lights {
                        Some(num_buttons)
                    } else {
                        None
                    }
                })
                .min()
                .unwrap()
        })
        .sum()
}

fn min_button_presses_joltage(machines: &[Machine]) -> u64 {
    machines
        .iter()
        .map(|machine| {
            assert!(machine.button_wiring.len() < 32);
            let optimizer = Optimize::new();

            let count_vars = (0..machine.joltage_req.len())
                .map(|idx| Int::fresh_const(&format!("count_{idx}")))
                .collect::<Vec<_>>();

            let button_presses = (0..machine.button_wiring.len())
                .map(|idx| Int::fresh_const(&format!("button_{idx}")))
                .collect::<Vec<_>>();

            for (count_var, joltage_req) in iter::zip(count_vars.iter(), machine.joltage_req.iter())
            {
                optimizer.assert(&count_var.eq(*joltage_req));
            }

            for button_press in &button_presses {
                optimizer.assert(&button_press.ge(0));
            }

            for (count_idx, count_var) in count_vars.iter().enumerate() {
                let button_sum = Int::add(
                    &machine
                        .button_wiring
                        .iter()
                        .enumerate()
                        .filter(|(_, wiring)| *wiring.get(count_idx).unwrap())
                        .map(|(button_idx, _)| &button_presses[button_idx])
                        .collect::<Vec<_>>(),
                );
                optimizer.assert(&count_var.eq(&button_sum));
            }

            let num_button_presses = Int::add(&button_presses);
            optimizer.minimize(&num_button_presses);

            assert_eq!(optimizer.check(&[]), SatResult::Sat);
            optimizer
                .get_model()
                .unwrap()
                .eval(&num_button_presses, false)
                .unwrap()
                .as_u64()
                .unwrap()
        })
        .sum()
}

pub fn solve_day10(input_data: String) {
    let input = match parse_input(input_data) {
        Some(input) => input,
        None => {
            eprintln!("Could not parse input!");
            return;
        }
    };

    let start_1 = Instant::now();
    let sol_task1 = min_button_presses_light(&input);
    let end_1 = Instant::now();
    println!(
        "Task 1: {sol_task1}\tTook: {}µs",
        (end_1 - start_1).as_micros()
    );

    let start_2 = Instant::now();
    let sol_task2 = min_button_presses_joltage(&input);
    let end_2 = Instant::now();
    println!(
        "Task 2: {sol_task2}\tTook: {}µs",
        (end_2 - start_2).as_micros()
    );
}
