use std::io::Read;

fn main() -> Result<(), ()> {
    const FILE_PATH: &'static str = "input.txt";
    const IS_PART_TWO: bool = true;

    let Ok(mut file) = std::fs::File::open(FILE_PATH) else {
        return Err(());
    };

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let mut position = 50;
    let mut times_at_zero = 0;

    println!("Position: {position}");

    for line in buffer.lines() {
        let direction = line.chars().nth(0).unwrap();
        let distance = line[1..line.len()].parse::<i32>().unwrap();

        // Times the dial passed by zero
        let mut times_passed_zero = 0;

        // Don't cound the dial as passing by zero for the first 100 units of distance if it
        // started here
        let mut ignore_first_pass = position == 0;

        if direction == 'R' {
            position += distance;

            while position > 99 {
                // Gone past 99, wrap it around
                position = position - 100;

                if IS_PART_TWO {
                    // It wrapped around, but don't count passing zero if it landed on zero as
                    // we check that afterwards
                    let is_last_iteration = position >= 0 && position < 100;
                    if !ignore_first_pass && is_last_iteration && position != 0 {
                        times_passed_zero += 1;
                    }

                    ignore_first_pass = false;
                }
            }
        } else if direction == 'L' {
            position -= distance;

            while position < 0 {
                // Gone under 0, wrap back over
                position = position + 100;

                if IS_PART_TWO {
                    // It wrapped around, but don't count passing zero if it landed on zero as
                    // we check that afterwards
                    let is_last_iteration = position >= 0 && position < 100;
                    if !ignore_first_pass && !is_last_iteration && position != 0 {
                        times_passed_zero += 1;
                    }

                    ignore_first_pass = false;
                }
            }
        }

        if position == 0 {
            times_at_zero += 1;
        }

        // How many times the dial passed zero while spinning
        times_at_zero += times_passed_zero;

        println!(
            "{direction} {distance} Position: {position}    time passed zero: {times_passed_zero}   times at zero: {times_at_zero}"
        );
    }

    println!("Password: {times_at_zero}");

    Ok(())
}
