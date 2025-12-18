use std::collections::HashMap;

pub mod part1;
pub mod part2;

pub static SVR: usize = id_to_index(b"svr");
pub static YOU: usize = id_to_index(b"you");
pub static DAC: usize = id_to_index(b"dac");
pub static FFT: usize = id_to_index(b"fft");
pub static OUT: usize = id_to_index(b"out");

pub fn parse_devices_hashmap(input: &str) -> HashMap<[u8; 3], Vec<[u8; 3]>> {
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

pub const fn id_to_index(id: &[u8]) -> usize {
    (id[0] as usize - 97) * 26usize.pow(2) + (id[1] as usize - 97) * 26 + (id[2] as usize - 97)
}

pub fn index_to_id(index: usize) -> String {
    format!(
        "{}{}{}",
        (index / (26 * 26) % 26 + 97) as u8 as char,
        (index / 26 % 26 + 97) as u8 as char,
        (index % 26 + 97) as u8 as char,
    )
}

pub fn parse_devices_vec(input: &str) -> Vec<Vec<usize>> {
    let mut id_to_outputs = Vec::new();
    id_to_outputs.resize(26 * 26 * 26, Vec::with_capacity(4));
    for (id, outputs) in input.lines().map(|line| {
        let bytes = line.as_bytes();
        let id = id_to_index(bytes[..3].try_into().unwrap());
        let output_ids = bytes[5..].split(|&byte| byte == b' ').map(id_to_index);
        (id, output_ids)
    }) {
        id_to_outputs[id].extend(outputs);
    }
    id_to_outputs
}

fn path_count(outputs: &[Vec<usize>], counts: &mut Vec<u64>, node: usize) -> u64 {
    if counts[node] != u64::MAX {
        return counts[node];
    }

    counts[node] = outputs[node]
        .iter()
        .map(|&output| path_count(outputs, counts, output))
        .sum();

    counts[node]
}

pub fn dfs(outputs: &[Vec<usize>], counts: &mut Vec<u64>, start: usize, end: usize) -> u64 {
    counts[start] = u64::MAX;
    counts.iter_mut().for_each(|count| {
        if *count == 0 {
            *count = u64::MAX
        }
    });
    counts[end] = 1;
    path_count(outputs, counts, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_to_index() {
        let mut index_actual = 0;
        for i1 in b'a'..=b'z' {
            for i2 in b'a'..=b'z' {
                for i3 in b'a'..=b'z' {
                    let id = &[i1, i2, i3];
                    let index_generated = id_to_index(id);
                    assert_eq!(index_generated, index_actual);
                    index_actual += 1;
                }
            }
        }
    }

    #[test]
    fn test_dfs_part1() {
        let input = include_str!("../input-test-part1.txt");
        let expected = 5;
        let outputs = parse_devices_vec(input);
        let mut counts = vec![u64::MAX; 26_usize.pow(3)];
        let result = dfs(&outputs, &mut counts, YOU, OUT);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dfs_part2() {
        let input = include_str!("../input-test-part2.txt");
        let expected = 8;
        let outputs = parse_devices_vec(input);
        let mut counts = vec![u64::MAX; 26_usize.pow(3)];
        let result = dfs(&outputs, &mut counts, SVR, OUT);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dfs_test_input() {
        let input = include_str!("../input-test.txt");
        let expected = 5;
        let outputs = parse_devices_vec(input);
        let mut counts = vec![u64::MAX; 26_usize.pow(3)];
        let result = dfs(&outputs, &mut counts, YOU, OUT);
        assert_eq!(result, expected);
    }
}
