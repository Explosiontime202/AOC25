use day01::solve_day01;
use day02::solve_day02;
use day03::solve_day03;

mod day01;
mod day02;
mod day03;

fn main() {
    let mut args = std::env::args();
    if args.len() != 3 {
        println!("Usage: {} <day> input.txt", args.next().unwrap());
        return;
    }
    let mut args = args.skip(1);
    let day_str= args.next().unwrap();
    let day: u64 = match day_str.parse() {
        Ok(day) => day,
        Err(_) => {
            println!("Cannot parse day \"{day_str}\"!");
            return;
        }
    };
    let input_file = args.next().unwrap();
    let input_path = format!("day{day:02}/{input_file}");

    let input_data = match std::fs::read_to_string(&input_path) {
        Ok(str) => str,
        Err(err) => {
            println!(
                "Failed to read file (\"{input_path}\") due to {}",
                err.to_string()
            );
            return;
        }
    };

    match day {
        1 => solve_day01(input_data),
        2 => solve_day02(input_data),
        3 => solve_day03(input_data),
        _ if day > 12 => {
            println!("Unavailable day!");
            return;
        }
        _ => todo!("Unsolved day")
    }

    
}
