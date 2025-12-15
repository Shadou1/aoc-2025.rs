use std::collections::HashMap;

pub mod part1;
pub mod part2;

pub fn parse_devices(input: &str) -> HashMap<[u8; 3], Vec<[u8; 3]>> {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let id = bytes[..3].try_into().unwrap();
            let outputs = bytes[5..]
                .split(|&byte| byte == b' ')
                .map(|output| output.try_into().unwrap())
                .collect();
            (id, outputs)
        })
        .collect()
}

pub fn id_to_index(id: [u8; 3]) -> usize {
    (id[0] as usize - 97) * 26usize.pow(2) + (id[1] as usize - 97) * 26 + (id[2] as usize - 97)
}

pub fn parse_devices_vec(input: &str) -> Vec<Vec<usize>> {
    let mut id_to_outputs = Vec::new();
    id_to_outputs.resize(26 * 26 * 26, Vec::with_capacity(4));
    for (id, outputs) in input.lines().map(|line| {
        let bytes = line.as_bytes();
        let id = id_to_index(bytes[..3].try_into().unwrap());
        let output_ids = bytes[5..]
            .split(|&byte| byte == b' ')
            .map(|bytes| id_to_index(bytes.try_into().unwrap()));
        (id, output_ids)
    }) {
        id_to_outputs[id].extend(outputs);
    }
    id_to_outputs
}

#[cfg(test)]
mod tests {
    use super::*;
}
