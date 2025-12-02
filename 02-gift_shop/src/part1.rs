use crate::{parse_ranges, sum_invalid_ids};

pub fn solution(input: &str) -> u64 {
    let mut invalid_id_sum = 0;

    for (start, end) in parse_ranges(input) {
        // Only consider id's of even length and repeating numbers of half id's length
        let id_length = (start.ilog10() + 1) + (start.ilog10() + 1) % 2;
        let repeating_num_length = id_length / 2;
        println!("Range: {start}..={end} length: {repeating_num_length}");

        invalid_id_sum += sum_invalid_ids(
            start.max(10u64.pow(id_length - 1)),
            end.min(10u64.pow(id_length) - 1),
            repeating_num_length,
            None,
        );
    }

    invalid_id_sum
}

pub fn solution_simple(input: &str) -> u64 {
    let mut invalid_id_sum = 0;
    parse_ranges(input).for_each(|(start, end)| {
        let size = ((start as f64).log10() / 2.0).ceil().max(1.0) as usize;
        for i in start..=end {
            if i.ilog10() % 2 == 0 {
                continue;
            }
            if (i / 10u32.pow(size as u32) as u64) == (i % 10u32.pow(size as u32) as u64) {
                println!("{i}");
                invalid_id_sum += i;
            }
        }
    });

    invalid_id_sum
}

#[cfg(test)]
mod tests {
    use super::*;
}
