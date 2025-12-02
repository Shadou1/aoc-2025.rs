use crate::{parse_ranges, sum_invalids};

pub fn solution(input: &str) -> u64 {
    let mut invalid_id_sum = 0;

    let ranges = parse_ranges(input);
    for (start, end) in ranges {
        let start_size = start.ilog10() + 1;
        let end_size = end.ilog10() + 1;
        println!("Range: {start}..={end} start: {start_size} end: {end_size}");

        for num_size in start_size..=end_size {
            let mut prev_repeating_num_size = None;
            for repeating_num_size in 1..num_size {
                if num_size % repeating_num_size != 0 {
                    continue;
                }
                invalid_id_sum += sum_invalids(
                    start.max(10u64.pow(num_size - 1)),
                    end.min(10u64.pow(num_size) - 1),
                    repeating_num_size,
                    prev_repeating_num_size,
                );
                prev_repeating_num_size = Some(repeating_num_size);
            }
        }
    }

    invalid_id_sum
}

#[cfg(test)]
mod tests {
    use super::*;
}
