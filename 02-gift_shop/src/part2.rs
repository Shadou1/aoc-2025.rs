use crate::{parse_ranges, sum_invalid_ids};

pub fn solution(input: &str) -> u64 {
    let mut invalid_id_sum = 0;

    for (start, end) in parse_ranges(input) {
        let start_length = start.ilog10() + 1;
        let end_length = end.ilog10() + 1;
        println!("Range: {start}..={end} start_length: {start_length} end_length: {end_length}");

        for id_length in start_length..=end_length {
            let mut prev_repeating_num_length = None;
            for repeating_num_length in 1..id_length {
                if id_length % repeating_num_length != 0 {
                    continue;
                }
                invalid_id_sum += sum_invalid_ids(
                    start.max(10u64.pow(id_length - 1)),
                    end.min(10u64.pow(id_length) - 1),
                    repeating_num_length,
                    prev_repeating_num_length,
                );
                prev_repeating_num_length = Some(repeating_num_length);
            }
        }
    }

    invalid_id_sum
}

#[cfg(test)]
mod tests {
    use super::*;
}
