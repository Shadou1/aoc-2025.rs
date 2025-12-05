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
    for ingridient in input
        .lines()
        .skip(ranges.len() + 1)
        .map(|line| line.parse::<u64>().unwrap())
    {
        for range in merged_ranges.iter() {
            if range.contains(&ingridient) {
                fresh_count += 1;
                break;
            }
        }
    }

    fresh_count
}

#[cfg(test)]
mod tests {
    use super::*;
}
