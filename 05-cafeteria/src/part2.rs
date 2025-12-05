use std::ops::RangeInclusive;

use crate::merge_ranges;

pub fn solution(input: &str) -> u64 {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    for range in input.lines().take_while(|line| !line.is_empty()) {
        let (start, end) = range.split_once('-').unwrap();
        ranges.push(start.parse().unwrap()..=end.parse().unwrap());
    }
    let merged_ranges = merge_ranges(&mut ranges);

    let mut fresh_count = 0;
    for range in merged_ranges {
        fresh_count += range.end() - range.start() + 1;
    }

    fresh_count
}

#[cfg(test)]
mod tests {
    use super::*;
}
