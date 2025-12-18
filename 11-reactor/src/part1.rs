use std::collections::HashMap;

use crate::{OUT, YOU, dfs, id_to_index, parse_devices_hashmap, parse_devices_vec};

pub fn solution_hashmap(input: &str) -> u64 {
    let devices = parse_devices_hashmap(input);
    count_out_hashmap(&devices)
}

pub fn count_out_hashmap(devices: &HashMap<[u8; 3], Vec<[u8; 3]>>) -> u64 {
    let mut current_devices = vec![*b"you"];
    let mut count = 0;
    while let Some(device) = current_devices.pop() {
        if device == *b"out" {
            count += 1;
        } else {
            current_devices.extend_from_slice(devices.get(&device).unwrap());
        }
    }
    count
}

pub fn solution_vec(input: &str) -> u64 {
    let devices = parse_devices_vec(input);
    count_out_vec(&devices)
}

pub fn count_out_vec(devices: &[Vec<usize>]) -> u64 {
    let mut count = 0;
    let out = id_to_index(b"out");

    let mut current_devices: Vec<usize> = Vec::new();
    current_devices.extend_from_slice(&devices[id_to_index(b"you")]);

    while let Some(device) = current_devices.pop() {
        if device == out {
            count += 1;
        } else {
            current_devices.extend_from_slice(&devices[device]);
        }
    }

    count
}

pub fn solution_dfs(input: &str) -> u64 {
    let outputs = parse_devices_vec(input);
    let mut counts = vec![u64::MAX; 26_usize.pow(3)];
    dfs(&outputs, &mut counts, YOU, OUT)
}

#[cfg(test)]
mod tests {
    use super::*;
}
