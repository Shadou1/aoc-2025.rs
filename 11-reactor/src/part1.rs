use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use crate::{bfs, dfs, id_to_index, index_to_id, parse_devices_hashmap, parse_devices_vec};

pub fn solution(input: &str) -> u64 {
    let devices = parse_devices_hashmap(input);
    count_out(&devices)
}

pub fn count_out(devices: &HashMap<[u8; 3], Vec<[u8; 3]>>) -> u64 {
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

pub fn solution_bfs(input: &str) -> u64 {
    let devices = parse_devices_vec(input);
    let mut counts: Vec<u64> = vec![0; 26 * 26 * 26];
    let mut visited: Vec<bool> = vec![false; 26 * 26 * 26];
    let start = Path {
        index: YOU,
        count: 1,
        inputs: HashSet::new(),
    };
    visited[YOU] = true;
    let result = bfs_device(&devices, start, OUT, &mut counts, &mut visited);
    result.count
}

pub fn solution_bfs2(input: &str) -> u64 {
    let outputs = parse_devices_vec(input);
    let count = bfs(&outputs, YOU, OUT);
    count
}

pub fn solution_dfs(input: &str) -> u64 {
    let outputs = parse_devices_vec(input);
    let mut counts = vec![u64::MAX; 26_usize.pow(3)];
    let count = dfs(&outputs, &mut counts, YOU, OUT);
    count
}

pub fn count_svr_to_out(input: &str) -> u64 {
    let devices = parse_devices_vec(input);
    let mut counts: Vec<u64> = vec![0; 26 * 26 * 26];
    let mut visited: Vec<bool> = vec![false; 26 * 26 * 26];
    let start = Path {
        index: SVR,
        count: 1,
        inputs: HashSet::new(),
    };
    visited[SVR] = true;
    let result = bfs_device(&devices, start, OUT, &mut counts, &mut visited);
    result.count
}

#[derive(Clone)]
pub struct Path {
    pub index: usize,
    pub count: u64,
    pub inputs: HashSet<usize>,
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inputs: Vec<String> = self
            .inputs
            .iter()
            .map(|&input| index_to_id(input))
            .collect();
        write!(
            f,
            "{}: {}; inputs: {:?}",
            index_to_id(self.index),
            self.count,
            inputs,
        )
    }
}

pub static SVR: usize = id_to_index(b"svr");
pub static YOU: usize = id_to_index(b"you");
pub static OUT: usize = id_to_index(b"out");

pub fn bfs_device(
    devices: &[Vec<usize>],
    start: Path,
    end: usize,
    counts: &mut [u64],
    visited: &mut [bool],
) -> Path {
    let mut current_paths: Vec<Path> = Vec::new();
    current_paths.push(start.clone());

    let mut input_muls: Vec<u64> = vec![0; 26 * 26 * 26];
    input_muls[start.index] = 1;
    // input_muls[end] = 1;

    let mut reached_path: Option<Path> = None;

    let mut step = 0;

    // breath search dac and fft devices
    while !current_paths.is_empty() {
        step += 1;
        println!("{step}");
        // explore outputs
        let current_len = current_paths.len();
        for i in 0..current_len {
            for (output_i, &output) in devices[current_paths[i].index].iter().enumerate() {
                if !visited[output] {
                    // visiting the node first time
                    println!("Not visited:\t{}", index_to_id(output));
                    // counts[output] = current_paths[i].count;
                    visited[output] = true;
                    // input_muls[current_paths[i].index] += 1;
                    // input_muls[output] = input_muls[current_paths[i].index];
                    input_muls[output] = 1;

                    let mut inputs = current_paths[i].inputs.clone();
                    if inputs.contains(&current_paths[i].index) {
                        panic!("Adding circular or already visited device");
                    }
                    inputs.insert(current_paths[i].index);
                    let new_path = Path {
                        index: output,
                        count: current_paths[i].count,
                        inputs,
                    };

                    if output_i > 0 {
                        new_path
                            .inputs
                            .iter()
                            .for_each(|&input| input_muls[input] += new_path.count);
                        // input_muls[current_paths[i].index] = 1;
                    }

                    if output == end {
                        // new_path
                        //     .inputs
                        //     .iter()
                        //     .for_each(|&input| input_muls[input] += 1);
                        reached_path = Some(new_path);
                    } else {
                        current_paths.push(new_path);
                    }
                } else {
                    // some other path visited this earlier
                    println!("Already visited:\t{}", index_to_id(output));
                    // counts[output] += current_paths[i].count;
                    for j in 0..current_paths.len() {
                        // if i == j {
                        //     continue;
                        // }
                        if current_paths[j].index == output
                            || current_paths[j].inputs.contains(&output)
                        {
                            // // inputs already part of current_paths[j]
                            // let old_inputs: HashSet<usize> = current_paths[j]
                            //     .inputs
                            //     .intersection(&current_paths[i].inputs)
                            //     .copied()
                            //     .collect();
                            // old_inputs.iter().for_each(|&input| input_muls[input] += 1);
                            // // inputs not part of current_paths[j]
                            // let new_inputs: HashSet<usize> = current_paths[j]
                            //     .inputs
                            //     .difference(&current_paths[i].inputs)
                            //     .copied()
                            //     .collect();
                            // new_inputs.iter().for_each(|&input| input_muls[input] += 1);

                            // current_paths[j]
                            //     .inputs
                            //     .iter()
                            //     .for_each(|&input| input_muls[input] = input_muls[input].max(1));
                            // input_muls[current_paths[j].index] = input_muls[current_paths[j].index].max(1);

                            // current_paths[i]
                            //     .inputs
                            //     .iter()
                            //     .for_each(|&input| input_muls[input] += 1);
                            // input_muls[current_paths[i].index] += 1;

                            let mut all_inputs: HashSet<usize> = current_paths[j]
                                .inputs
                                .union(&current_paths[i].inputs)
                                .copied()
                                .collect();
                            all_inputs.insert(current_paths[i].index);

                            current_paths[j].inputs = all_inputs;
                            current_paths[j].count += current_paths[i].count * input_muls[output];

                            // if output_i > 0 {
                            //     current_paths[i]
                            //         .inputs
                            //         .iter()
                            //         .for_each(|&input| input_muls[input] += 1);
                            //     input_muls[current_paths[i].index] += 1;
                            // }

                            // current_paths[j].count += current_paths[i].count * input_muls[output];
                            // current_paths[j].count +=
                            //     current_paths[i].count * input_muls[current_paths[i].index];
                        }
                    }
                    if let Some(reached_path) = &mut reached_path
                        && (reached_path.index == output || reached_path.inputs.contains(&output))
                    {
                        // inputs already part of current_paths[j]
                        // let old_inputs: HashSet<usize> = reached_path
                        //     .inputs
                        //     .intersection(&current_paths[i].inputs)
                        //     .copied()
                        //     .collect();
                        // old_inputs.iter().for_each(|&input| input_muls[input] += 1);
                        // inputs not part of current_paths[j]
                        // let new_inputs: HashSet<usize> = reached_path
                        //     .inputs
                        //     .difference(&current_paths[i].inputs)
                        //     .copied()
                        //     .collect();
                        // new_inputs.iter().for_each(|&input| input_muls[input] += 1);

                        // reached_path
                        //     .inputs
                        //     .iter()
                        //     .for_each(|&input| input_muls[input] = input_muls[input].max(1));
                        // input_muls[reached_path.index] = input_muls[reached_path.index].max(1);

                        // current_paths[i]
                        //     .inputs
                        //     .iter()
                        //     .for_each(|&input| input_muls[input] += 1);
                        // input_muls[current_paths[i].index] += 1;

                        // if output_i > 0 {
                        //     current_paths[i]
                        //         .inputs
                        //         .iter()
                        //         .for_each(|&input| input_muls[input] *= 2);
                        //     input_muls[current_paths[i].index] *= 2;
                        // }

                        let mut all_inputs: HashSet<usize> = reached_path
                            .inputs
                            .union(&current_paths[i].inputs)
                            .copied()
                            .collect();
                        all_inputs.insert(current_paths[i].index);

                        reached_path.inputs = all_inputs;
                        reached_path.count += current_paths[i].count;

                        // reached_path.count += current_paths[i].count * input_muls[output];
                        // reached_path.count +=
                        //     current_paths[i].count * input_muls[current_paths[i].index];

                        // reached_path.count += current_paths[i].count;
                        // reached_path.inputs = reached_path
                        //     .inputs
                        //     .union(&current_paths[i].inputs)
                        //     .copied()
                        //     .collect();
                    }
                }

                print!("[ ");
                for (i, outputs) in devices.iter().enumerate() {
                    if !outputs.is_empty() && input_muls[i] != 0 {
                        print!("{}: {}, ", index_to_id(i), input_muls[i]);
                    }
                }
                println!("]");
            }
        }

        // update counts
        // for path in &mut current_paths {
        //     path.count =
        //         path.inputs.iter().map(|&index| counts[index]).sum::<u64>() + counts[path.index];
        //     // for &input in &path.inputs {
        //     //     if counts[input] > path.count {
        //     //         path.count = counts[input]
        //     //     }
        //     // }
        //     // if counts[path.index] > path.count {
        //     //     path.count = counts[path.index]
        //     // }
        // }
        // if let Some(reached_path) = &mut reached_path {
        //     reached_path.count = reached_path
        //         .inputs
        //         .iter()
        //         .map(|&index| counts[index])
        //         .sum::<u64>()
        //         + counts[reached_path.index];
        // }
        // for path in &mut reached_paths {
        //     path.count =
        //         path.inputs.iter().map(|&index| counts[index]).sum::<u64>() + counts[path.index];
        // for &input in &path.inputs {
        //     if counts[input] > path.count {
        //         path.count = counts[input]
        //     }
        // }
        // if counts[path.index] > path.count {
        //     path.count = counts[path.index]
        // }
        // }

        // remove old paths
        current_paths = current_paths[current_len..].to_vec();

        println!("{:?}", current_paths);
        println!("{:?}", reached_path);

        // print!("[ ");
        // for (i, outputs) in devices.iter().enumerate() {
        //     if !outputs.is_empty() {
        //         print!("{}: {}, ", index_to_id(i), input_muls[i]);
        //     }
        // }
        // println!("]");

        println!();

        // remove reached paths
        // current_paths.retain(|path| {
        //     if end == path.index {
        //         reached_path = Some(path.clone());
        //         false
        //     } else {
        //         true
        //     }
        // });
    }

    // for path in reached_paths {
    //     println!("Reached path count: {}", path.count);
    // }
    // println!("Reached path: {:?}", reached_path);
    // set count to 0 on every device not touching path
    // for reached in &reached_paths {
    //     for (i, count) in counts.iter_mut().enumerate() {
    //         if !reached.inputs.contains(&i) {
    //             *count = 0;
    //         }
    //     }
    // }
    visited.fill(false);
    let Some(reached_path) = reached_path else {
        panic!("Couldn't reach device");
    };
    for &i in &reached_path.inputs {
        visited[i] = true;
    }
    visited[reached_path.index] = true;

    // end.into_iter().map(|end| counts[end]).collect()
    reached_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_svr_to_out() {
        let input = include_str!("../input-test-part2.txt");
        let expected = 8;
        let result = count_svr_to_out(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input-test.txt");
        let expected = 5;
        let result = solution(input);
        assert_eq!(result, expected);
        let result = solution_vec(input);
        assert_eq!(result, expected);
        let result = solution_bfs(input);
        assert_eq!(result, expected);
    }
}
