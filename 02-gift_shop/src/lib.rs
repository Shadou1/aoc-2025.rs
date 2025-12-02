pub mod part1;
pub mod part2;

pub fn parse_ranges(ranges: &str) -> impl Iterator<Item = (u64, u64)> {
    ranges.trim().split_terminator(',').map(|range| {
        let (start, end) = range.split_once('-').unwrap();
        (start.parse().unwrap(), end.parse().unwrap())
    })
}

pub fn sum_invalids(start: u64, end: u64, size: u32, prev_size: Option<u32>) -> u64 {
    let mut invalid_id_sum = 0;

    println!("\tPassed: {start}..={end}, size: {size}");
    let mut step = 1;
    for i in 1..((start.ilog10() + 1) / size) {
        step += 10u64.pow(size).pow(i);
    }
    let mut prev_step = None;
    if let Some(prev_size) = prev_size {
        prev_step = Some(1);
        for i in 1..((start.ilog10() + 1) / prev_size) {
            prev_step = prev_step.map(|prev_step| prev_step + 10u64.pow(prev_size).pow(i));
        }
    }
    let first_invalid = start.div_ceil(step).max(10u64.pow(size - 1)) * step;
    println!("\tActual: {first_invalid}..={end}, step: {step}, prev_step: {prev_step:?}");
    print!("\tInvalids: ");
    for invalid in (first_invalid..=end).step_by(step as usize) {
        if let Some(prev_step) = prev_step
            && invalid % prev_step == 0
        {
            // Already counted on the previous step
            continue;
        }
        print!("{invalid}, ");
        invalid_id_sum += invalid;
    }
    println!();

    invalid_id_sum
}

#[cfg(test)]
mod tests {
    use super::*;
}
