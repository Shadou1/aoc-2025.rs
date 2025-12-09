use std::collections::BinaryHeap;

use crate::{get_sorted_areas, parse_points};

pub fn solution(input: &str) -> u64 {
    let points = parse_points(input);
    let areas = get_sorted_areas(&points);
    get_largest_area(&areas)
}

pub fn get_largest_area(areas: &BinaryHeap<u64>) -> u64 {
    *areas.peek().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

}
