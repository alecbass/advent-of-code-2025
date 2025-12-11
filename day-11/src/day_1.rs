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

// Finds if a device is connected to the "out" device and returns the amount of unique paths to get
// to the "out" device
fn find_ways_to_out(graph: &HashMap<String, Vec<String>>, device_id: &str) -> i32 {
    let connected_device_ids = graph.get(device_id).unwrap();
    let mut paths_count = 0;

    for connected_device_id in connected_device_ids {
        if connected_device_id == "out" {
            // Found the end device, return true that previous devices are connected
            return 1;
        }

        // Find if connected devices can reach the "out" device
        paths_count += find_ways_to_out(graph, connected_device_id);
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

    let paths_count = find_ways_to_out(&graph, "you");
    Ok(paths_count)
}
