use std::ops::RangeInclusive;

pub mod part1;
pub mod part2;

pub fn merge_ranges(ranges: &mut [RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
    ranges.sort_by_key(|range| *range.start());
    let mut merged_ranges = vec![ranges[0].clone()];

    for curr_range in &ranges[1..] {
        let prev_range = &merged_ranges[merged_ranges.len() - 1];
        if prev_range.end() >= curr_range.end() {
            // current range is within previous range
            continue;
        } else if curr_range.start() <= prev_range.end() {
            // current range starts in previous range and ends outside of previous range
            let merged_ranges_len = merged_ranges.len();
            merged_ranges[merged_ranges_len - 1] = *prev_range.start()..=*curr_range.end();
        } else {
            // current range is after the previous range
            merged_ranges.push(curr_range.clone());
        }
    }

    merged_ranges
}

#[cfg(test)]
mod tests {
    use super::*;
}
