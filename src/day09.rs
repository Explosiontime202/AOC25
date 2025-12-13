use std::{fmt::Display, time::Instant};

#[derive(Debug, Clone, Copy)]
struct Point2D {
    x: u64,
    y: u64,
}

impl Point2D {
    fn area(a: &Self, b: &Self) -> u64 {
        let len_x = a.x.max(b.x) - a.x.min(b.x) + 1;
        let len_y = a.y.max(b.y) - a.y.min(b.y) + 1;
        len_x * len_y
    }

    fn idx(&self, width: u64) -> u64 {
        self.x + width * self.y
    }
}

impl TryFrom<&str> for Point2D {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut values = value.split(",");
        let Some(x_str) = values.next() else {
            return Err(());
        };
        let Some(y_str) = values.next() else {
            return Err(());
        };
        let Ok(x) = x_str.parse() else {
            return Err(());
        };
        let Ok(y) = y_str.parse() else {
            return Err(());
        };
        Ok(Point2D { x, y })
    }
}

impl Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[must_use]
fn parse_input(input_data: String) -> Option<Vec<Point2D>> {
    input_data
        .lines()
        .map(Point2D::try_from)
        .map(Result::ok)
        .collect()
}

#[must_use]
fn largest_rec(points: &[Point2D]) -> u64 {
    (0..points.len())
        .flat_map(|i| ((i + 1)..points.len()).map(move |j| (i, j)))
        .map(|(i, j)| {
            let a = &points[i];
            let b = &points[j];
            Point2D::area(a, b)
        })
        .max()
        .unwrap()
}

#[must_use]
fn solve_task2(points: &[Point2D]) -> u64 {
    let mut max_area = 0;
    for i in 0..points.len() {
        let a = &points[i];
        for j in i + 1..points.len() {
            let b = &points[j];

            let min_x = a.x.min(b.x);
            let min_y = a.y.min(b.y);
            let max_x = a.x.max(b.x);
            let max_y = a.y.max(b.y);

            let mut invalid_rec = false;
            let mut idx = (i + 1) % points.len();

            while ((idx + 1) % points.len()) != i {
                if idx == j || ((idx + 1) % points.len()) == j {
                    idx = (idx + 1) % points.len();
                    continue;
                }

                let line_a = &points[idx];
                let line_b = &points[(idx + 1) % points.len()];

                // check if one of the vertices is inside the rect
                if line_a.x > min_x && line_a.x < max_x && line_a.y > min_y && line_a.y < max_y {
                    invalid_rec = true;
                    break;
                }
                if line_b.x > min_x && line_b.x < max_x && line_b.y > min_y && line_b.y < max_y {
                    invalid_rec = true;
                    break;
                }

                let line_min_x = line_a.x.min(line_b.x);
                let line_max_x = line_a.x.max(line_b.x);
                let line_min_y = line_a.y.min(line_b.y);
                let line_max_y = line_a.y.max(line_b.y);

                // check if line crosses through rectangle
                if line_min_x <= min_x && line_max_x >= max_x && line_min_x != line_max_x {
                    debug_assert_eq!(line_a.y, line_b.y);
                    if line_a.y > min_y && line_a.y < max_y {
                        invalid_rec = true;
                        break;
                    }
                }
                if line_min_y <= min_y && line_max_y >= max_y && line_min_y != line_max_y {
                    debug_assert_eq!(line_a.x, line_b.x);
                    if line_a.x > min_x && line_a.x < max_x {
                        invalid_rec = true;
                        break;
                    }
                }
                idx = (idx + 1) % points.len();
            }

            if invalid_rec {
                continue;
            }

            max_area = max_area.max(Point2D::area(a, b));
        }
    }

    max_area
}

#[allow(unused)]
fn print_grid(points: &[Point2D]) {
    let width = points.iter().map(|p| p.x).max().unwrap() + 1;
    let height = points.iter().map(|p| p.y).max().unwrap() + 1;
    let mut points_vec = points.iter().copied().collect::<Vec<_>>();
    points_vec.sort_unstable_by_key(|p| p.y * width + p.x);

    let mut out_idx = 0;
    let mut advance_to = |target_idx: u64, is_point: bool| {
        while out_idx < target_idx {
            print!(".");
            out_idx += 1;
            if out_idx % width == 0 {
                print!("\n");
            }
        }
        if is_point {
            print!("#");
        } else {
            print!(".");
        }
        out_idx += 1;
        if out_idx % width == 0 {
            print!("\n");
        }
    };

    for p in points_vec {
        advance_to(p.idx(width), true);
    }
    advance_to(width * height, false);
}

pub fn solve_day09(input_data: String) {
    let input = match parse_input(input_data) {
        Some(input) => input,
        None => {
            eprintln!("Could not parse input!");
            return;
        }
    };

    let start_1 = Instant::now();
    let sol_task1 = largest_rec(&input);
    let end_1 = Instant::now();
    println!(
        "Task 1: {sol_task1}\tTook: {}µs",
        (end_1 - start_1).as_micros()
    );

    let start_2 = Instant::now();
    let sol_task2 = solve_task2(&input);
    let end_2 = Instant::now();
    println!(
        "Task 2: {sol_task2}\tTook: {}µs",
        (end_2 - start_2).as_micros()
    );

    // print_grid(&input);
}
