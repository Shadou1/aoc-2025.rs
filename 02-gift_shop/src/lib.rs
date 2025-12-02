pub mod part1;
pub mod part2;

pub fn parse_ranges(ranges: &str) -> impl Iterator<Item = (u64, u64)> {
    ranges.trim().split_terminator(',').map(|range| {
        let (start, end) = range.split_once('-').unwrap();
        (start.parse().unwrap(), end.parse().unwrap())
    })
}

/// Get sum of all invalid id's between `start` and `end` with specified repeating number length.
/// Pass previous repeating number length to avoid counting invalid id's multiple times.
pub fn sum_invalid_ids(
    start: u64,
    end: u64,
    repeating_num_length: u32,
    prev_repeating_num_length: Option<u32>,
) -> u64 {
    println!("\tPassed: {start}..={end}, length: {repeating_num_length}");
    // Calculate step between invalid ids
    let mut step = 1;
    for i in 1..((start.ilog10() + 1) / repeating_num_length) {
        step += 10u64.pow(repeating_num_length).pow(i);
    }
    // Calculate previous step between invalid numbers
    let mut prev_step = None;
    if let Some(prev_length) = prev_repeating_num_length {
        prev_step = Some(1);
        for i in 1..((start.ilog10() + 1) / prev_length) {
            prev_step = prev_step.map(|prev_step| prev_step + 10u64.pow(prev_length).pow(i));
        }
    }

    let first_invalid_id = start
        .div_ceil(step)
        .max(10u64.pow(repeating_num_length - 1))
        * step;
    println!("\tActual: {first_invalid_id}..={end}, step: {step}, prev_step: {prev_step:?}");
    let mut invalid_id_sum = 0;
    print!("\tInvalids: ");
    for invalid_id in (first_invalid_id..=end).step_by(step as usize) {
        if let Some(prev_step) = prev_step
            && invalid_id % prev_step == 0
        {
            // Already counted in the previous call to 'sum_invalid_ids'
            continue;
        }
        print!("{invalid_id}, ");
        invalid_id_sum += invalid_id;
    }
    println!();

    invalid_id_sum
}

#[cfg(test)]
mod tests {
    use super::*;
}
