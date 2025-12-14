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
    cumulative_path: Vec<&String>,
    contains_dac: bool, // Use a variable so we don't call Vec.contains constantly
    contains_fft: bool,
) -> i32 {
    let Some(connected_device_ids) = graph.get(device_id) else {
        // Reached "out", return zero as we're at the end
        return 0;
    };

    if let Some(first) = cumulative_path.first()
        && *first != "svr"
    {
        // Didn't start from "svr", return
        return 0;
    }

    let mut paths_count = 0;

    for connected_device_id in connected_device_ids {
        if connected_device_id == "out" && contains_dac && contains_fft {
            // Found the end device, return true that previous devices are connected
            paths_count += 1;
            println!("{connected_device_id} {cumulative_path:?}");
            continue;
        }

        let contains_dac = contains_dac || connected_device_id == "dac";
        let contains_fft = contains_fft || connected_device_id == "fft";

        // Find if connected devices can reach the "out" device
        let cumulative_path = cumulative_path
            .iter()
            .map(|id| *id)
            .chain(vec![connected_device_id])
            .collect::<Vec<&String>>();

        paths_count += find_svr_to_out(
            graph,
            connected_device_id,
            cumulative_path,
            contains_dac,
            contains_fft,
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

    for line in buffer.lines() {
        if line.is_empty() {
            continue;
        }

        let (device_id, to_device_ids) = parse_line(line)?;
        graph.insert(device_id, to_device_ids);
    }

    let paths_count = find_svr_to_out(&graph, "svr", vec![&"svr".to_owned()], false, false);
    Ok(paths_count)
}
