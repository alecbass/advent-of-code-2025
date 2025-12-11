use std::{collections::HashSet, i64, io::Read};

const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

fn get_ingredient_range(input: &str) -> (i64, i64) {
    let mut parts = input.split("-");
    let start = parts.next().unwrap().parse::<i64>().unwrap();
    let end = parts.next().unwrap().parse::<i64>().unwrap();
    (start, end)
}

fn main() {
    const FILE_PATH: &'static str = "input.txt";
    let mut file = std::fs::File::open(FILE_PATH).unwrap();

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    // buffer = TEST_INPUT.to_string();

    let lines = buffer.lines();
    let mut fresh_ingredient_ranges = HashSet::<(i64, i64)>::new();
    let mut fresh_ingredient_count = 0;
    let mut is_reading_ranges = true;

    for line in lines {
        if line.len() == 0 {
            // Swap to reading ingredient IDs
            is_reading_ranges = false;
            continue;
        }

        if is_reading_ranges {
            let (start, end) = get_ingredient_range(line);
            fresh_ingredient_ranges.insert((start, end));
        } else {
            let id = line.parse::<i64>().unwrap();

            for range in fresh_ingredient_ranges.iter() {
                // Check if the product ID is within any fresh ranges
                let (start, end) = range;

                if start <= &id && &id <= end {
                    // The ingredient is fresh if it's in ANY range, so exit on one successful
                    // check
                    fresh_ingredient_count += 1;
                    break;
                }
            }
        }
    }

    println!("Fresh ingredient count: {fresh_ingredient_count}");

    let mut fresh_ingredient_ids = HashSet::<i64>::new();

    let mut min_range = i64::MAX;
    let mut max_range = 0;

    for range in fresh_ingredient_ranges.iter() {
        let (start, end) = range;

        if *start < min_range {
            min_range = *start;
        }

        if *end > max_range {
            max_range = *end;
        }

        // for id in *start..=*end {
        //     fresh_ingredient_ids.insert(id);
        // }
    }

    let id_count = max_range - min_range;

    println!("{min_range} {max_range} {id_count}");

    println!("Fresh ingredient IDs: {fresh_ingredient_ids:?}");
    println!(
        "Fresh ingredient ID count: {}",
        fresh_ingredient_ranges.len()
    );

    // Terrible part two solution: Count every single fresh ID
}
