use std::{collections::BinaryHeap, iter};

pub mod part1;
pub mod part2;

pub struct Point {
    x: u64,
    y: u64,
}

impl Point {
    pub fn manhattan_distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
    pub fn area(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

pub struct PointDistance {}

pub fn get_sorted_areas(points: &[Point]) -> BinaryHeap<u64> {
    // let mut areas = BinaryHeap::with_capacity(points.len() * 2);
    // for (i, point1) in points.iter().enumerate() {
    //     for point2 in &points[i..] {
    //         areas.push(point1.area(point2));
    //     }
    // }
    //
    // areas
    points
        .iter()
        .enumerate()
        .flat_map(|(i, point)| iter::repeat_n(point, points.len() - 1).zip(points[i..].iter()))
        .map(|(point1, point2)| point1.area(point2))
        .collect()
}

pub fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut nums = line
                .as_bytes()
                .splitn(2, |&byte| byte == b',')
                .map(|bytes| {
                    bytes
                        .iter()
                        .fold(0_u64, |acc, byte| acc * 10 + (byte - 48) as u64)
                });
            Point {
                x: nums.next().unwrap(),
                y: nums.next().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
}
