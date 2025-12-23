use std::{collections::HashMap, fs::File, io::Read};

type DeviceAndPath = (String, Vec<String>);

fn parse_line(line: &str) -> Result<DeviceAndPath, ()> {
    let mut split = line.split(":");
    let device_id = split.next().unwrap().to_string();

    // : aaa bbb ccc -> ["aaa", "bbb", "ccc"]
    let to_device_ids = split
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .map(|id| id.to_owned())
        .collect();

    Ok((device_id, to_device_ids))
}

// Finds the amount of devices that start at "svr", end at "out" and pass through both "dac" and
// "fft
fn find_svr_to_out(
    graph: &HashMap<String, Vec<String>>,
    device_id: &str,
    contains_dac: bool, // Use a variable so we don't call Vec.contains constantly
    contains_fft: bool,
    depth: i32,
) -> i32 {
    // println!("{device_id} {} {cumulative_path:?}");

    let mut paths_count = 0;
    let contains_dac = contains_dac || device_id == "dac";
    let contains_fft = contains_fft || device_id == "fft";

    // if device_id == "out" && contains_dac && contains_fft {
    //     // Found the end device, return true that previous devices are connected
    //     println!("{paths_count} {device_id}");
    //     return 1;
    // }

    if contains_dac && contains_fft {
        // This route passes through dac and fft, it should end at out so end looking here
        // Found the end device, return true that previous devices are connected
        println!("{paths_count} {device_id}");
        return 1;
    }

    let Some(connected_device_ids) = graph.get(device_id) else {
        // No path for some reason. This shouldn't be reached
        return 0;
    };

    for connected_device_id in connected_device_ids {
        paths_count += find_svr_to_out(
            graph,
            connected_device_id,
            contains_dac,
            contains_fft,
            depth + 1,
        );
    }

    paths_count
}

pub fn solve(mut input_file: File) -> Result<i32, ()> {
    let mut graph = HashMap::<String, Vec<String>>::new();
    let mut buffer = String::new();
    if let Err(_e) = input_file.read_to_string(&mut buffer) {
        return Err(());
    }

    //     buffer = "svr: aaa bbb
    // aaa: fft
    // fft: ccc
    // bbb: tty
    // tty: ccc
    // ccc: ddd eee
    // ddd: hub
    // hub: fff
    // eee: dac
    // dac: fff
    // fff: ggg hhh
    // ggg: out
    // hhh: out".to_owned();

    for line in buffer.lines() {
        if line.is_empty() {
            continue;
        }

        let (device_id, to_device_ids) = parse_line(line)?;
        graph.insert(device_id, to_device_ids);
    }

    let paths_count = find_svr_to_out(&graph, "svr", false, false, 0);
    Ok(paths_count)
}
