use crate::{DAC, FFT, OUT, SVR, dfs, parse_devices_vec};

pub fn solution_dfs(input: &str) -> u64 {
    let devices = parse_devices_vec(input);
    let mut counts = vec![u64::MAX; 26_usize.pow(3)];
    let count_svr_fft = dfs(&devices, &mut counts, SVR, FFT);
    let count_fft_dac = dfs(&devices, &mut counts, FFT, DAC);
    let count_dac_out = dfs(&devices, &mut counts, DAC, OUT);
    count_svr_fft * count_fft_dac * count_dac_out
}

#[cfg(test)]
mod tests {
    use super::*;
}
