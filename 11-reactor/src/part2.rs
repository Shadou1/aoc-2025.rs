use std::collections::HashSet;

use crate::{dfs, id_to_index, parse_devices_vec, part1::{Path, bfs_device}};

pub fn solution(input: &str) -> u64 {
    let devices = parse_devices_vec(input);
    count_out_bfs(&devices)
}

pub fn solution_dfs(input: &str) -> u64 {
    let devices = parse_devices_vec(input);
    let mut counts = vec![u64::MAX; 26_usize.pow(3)];
    let count_svr_fft = dfs(&devices, &mut counts, SVR, FFT);
    let count_fft_dac = dfs(&devices, &mut counts, FFT, DAC);
    let count_dac_out = dfs(&devices, &mut counts, DAC, OUT);
    count_svr_fft * count_fft_dac * count_dac_out
}

pub static SVR: usize = id_to_index(b"svr");
pub static OUT: usize = id_to_index(b"out");
pub static DAC: usize = id_to_index(b"dac");
pub static FFT: usize = id_to_index(b"fft");
pub static YOU: usize = id_to_index(b"you");

pub fn count_out_bfs(devices: &[Vec<usize>]) -> u64 {
    let mut counts: Vec<u64> = vec![0; 26 * 26 * 26];
    let mut visited: Vec<bool> = vec![false; 26 * 26 * 26];
    let start = Path {
        index: SVR,
        count: 1,
        inputs: HashSet::new(),
    };
    visited[SVR] = true;

    let result = bfs_device(devices, start, FFT, &mut counts, &mut visited);
    // let fft_count = result[0];
    // let mut counts: Vec<u64> = vec![0; 26 * 26 * 26];
    // counts[FFT] = fft_count;
    println!("Path at FFT: {:?}", result);
    let result = bfs_device(devices, result, DAC, &mut counts, &mut visited);
    // let dac_count = result[0];
    // let mut counts: Vec<u64> = vec![0; 26 * 26 * 26];
    // counts[DAC] = dac_count;
    println!("Path at DAC: {:?}", result);
    let result = bfs_device(devices, result, OUT, &mut counts, &mut visited);
    println!("Path at OUT: {:?}", result);

    // let mut counts: Vec<u64> = vec![0; 26 * 26 * 26];
    // counts[SVR] = 1;
    //
    // bfs_device(devices, vec![SVR], vec![DAC],  &mut counts);
    // let dac_count = counts[DAC];
    // let mut counts: Vec<u64> = vec![0; 26 * 26 * 26];
    // counts[DAC] = dac_count;
    // println!("Paths at DAC: {}", counts[DAC]);
    // bfs_device(devices, vec![DAC], vec![FFT],  &mut counts);
    // let fft_count = counts[FFT];
    // let mut counts: Vec<u64> = vec![0; 26 * 26 * 26];
    // counts[FFT] = fft_count;
    // println!("Paths at FFT: {}", counts[FFT]);
    // bfs_device(devices, vec![FFT], vec![OUT],  &mut counts);
    // println!("Paths at OUT: {}", counts[OUT]);

    // bfs_device(devices, vec![SVR],vec![DAC, FFT, OUT],  &mut counts);
    //
    // // all paths at dac didnt pass through fft, paths at fft didn't pass through dac
    // println!("Paths at DAC: {}", counts[DAC]);
    // println!("Paths at FFT: {}", counts[FFT]);
    //
    // bfs_device(devices, vec![DAC, FFT], vec![DAC, FFT, OUT], &mut counts);
    //
    // // all paths at dac pass through fft, paths at fft pass through dac
    // println!("Paths at DAC: {}", counts[DAC]);
    // println!("Paths at FFT: {}", counts[FFT]);
    //
    // bfs_device(devices, vec![FFT], vec![OUT], &mut counts);
    // // let counts = bfs_device(devices, vec![DAC, FFT], vec![counts[0], counts[1]], vec![OUT]);
    //
    // // all paths passed through dac and fft and ended at out
    // println!("Paths at DAC: {}", counts[OUT]);
    // // println!("Paths at FFT: {}", counts[1]);

    result.count
}

#[cfg(test)]
mod tests {
    use super::*;
}
