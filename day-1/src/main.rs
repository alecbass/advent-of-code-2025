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

    // let buffer = "L1000";

    for line in buffer.lines() {
        let direction = line.chars().nth(0).unwrap();
        let distance = line[1..line.len()].parse::<i32>().unwrap();

        // Niche cases to count here:
        // 1. Dial starts at zero: Don't count leaving zero the first time
        // 2. Dial ends at zero: Don't count landing on zero as a pass, because we check it later
        // 3. Any other pass counts as passing by zero and should increment the password

        // Case 1
        // let mut ignore_first_pass = position == 0;

        // Times the dial passed by zero
        let mut times_passed_zero = 0;
        let start_position = position;

        // Rust mod is a remainder operator, 01/12/2025 worst day of my life...
        if direction == 'R' {
            position += distance;
            times_passed_zero = position / 100;
            position = position.rem_euclid(100);
        } else if direction == 'L' {
            position -= distance;
            times_passed_zero = (100 - position) / 100;
            position = position.rem_euclid(-100);
        }

        if start_position == 0 && times_passed_zero > 0 {
            times_passed_zero -= 1;
        }

        if position == 0 {
            if times_passed_zero > 0 {
                times_passed_zero -= 1;
            }

            times_at_zero += 1;
        }

        // How many times the dial passed zero while spinning
        if IS_PART_TWO {
            times_at_zero += times_passed_zero;
        }

        println!(
            "{direction} {distance} {start_position}->{position}    time passed zero: {times_passed_zero}   times at zero: {times_at_zero} END"
        );
    }

    println!("Password: {times_at_zero}");

    Ok(())
}
