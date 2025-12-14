mod part_2;

use std::fs::File;

use part_2::solve as part_2_solve;

fn main() -> Result<(), ()> {
    const FILE_PATH: &'static str = "input.txt";

    let Ok(file) = File::open(FILE_PATH) else {
        return Err(());
    };

    let paths_count = part_2_solve(file)?;
    println!("{paths_count}");
    Ok(())
}
