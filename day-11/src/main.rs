mod day_1;

use std::fs::File;

use day_1::solve;

fn main() -> Result<(), ()> {
    const FILE_PATH: &'static str = "input.txt";

    let Ok(file) = File::open(FILE_PATH) else {
        return Err(());
    };

    let paths_count = solve(file)?;
    println!("{paths_count}");
    Ok(())
}
