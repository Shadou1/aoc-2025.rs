pub mod part1;
pub mod part2;

pub fn parse_ranges(ranges: &str) -> impl Iterator<Item = std::ops::RangeInclusive<u64>> {
    ranges.trim().split_terminator(',').map(|range| {
        let (start, end) = range.split_once('-').unwrap();
        start.parse().unwrap()..=end.parse().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
}
