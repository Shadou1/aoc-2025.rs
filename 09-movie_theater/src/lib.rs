#![feature(test)]
extern crate test;

use std::{cmp::Ordering, collections::BinaryHeap, fmt::Display, iter};

pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: u64,
    y: u64,
}

impl Point {
    pub fn area(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    point1: Point,
    point2: Point,
}

impl Rectangle {
    pub fn area(&self) -> u64 {
        (self.point1.x.abs_diff(self.point2.x) + 1) * (self.point1.y.abs_diff(self.point2.y) + 1)
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "R: ({} - {})", self.point1, self.point2)
    }
}

impl Ord for Rectangle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.area().cmp(&other.area())
    }
}

impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn get_sorted_areas(points: &[Point]) -> BinaryHeap<u64> {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, point)| iter::repeat_n(point, points.len() - i).zip(points[i..].iter()))
        .filter_map(|(point1, point2)| {
            // Exclude points on the same x/y axis
            if point1.x == point2.x || point1.y == point2.y {
                None
            } else {
                Some(point1.area(point2))
            }
        })
        .collect()
}

pub fn get_sorted_rectangles(points: &[Point]) -> BinaryHeap<Rectangle> {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, point)| iter::repeat_n(point, points.len() - i).zip(points[i..].iter()))
        .filter_map(|(point1, point2)| {
            // Exclude points on the same x/y axis
            if point1.x == point2.x || point1.y == point2.y {
                None
            } else {
                Some(Rectangle {
                    point1: *point1,
                    point2: *point2,
                })
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    point1: Point,
    point2: Point,
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "L: ({} - {})", self.point1, self.point2)
    }
}

impl Line {
    pub fn new(point1: Point, point2: Point) -> Self {
        // make sure that line always goes from left to right or from up to down
        if point1.x > point2.x || point1.y > point2.y {
            Self {
                point2: point1,
                point1: point2,
            }
        } else {
            Self { point1, point2 }
        }
    }

    pub fn is_crossing_rectangle(&self, rectangle: &Rectangle) -> bool {
        // Arrange rectangle points for calculating a crossing vertical line
        let (mut rectangle_x1, mut rectangle_x2) = match rectangle.point1.x as i64 - rectangle.point2.x as i64 {
            (..=-1) => (rectangle.point1.x, rectangle.point2.x),
            (1..) => (rectangle.point2.x, rectangle.point1.x),
            0 => panic!("rectangle is on the same y axis"),
        };
        let (mut rectangle_y1, mut rectangle_y2) = match rectangle.point1.y as i64 - rectangle.point2.y as i64 {
            (..=-1) => (rectangle.point2.y, rectangle.point1.y),
            (1..) => (rectangle.point1.y, rectangle.point2.y),
            0 => panic!("rectangle is on the same x axis"),
        };

        if self.point1.x == self.point2.x {
            // Vertical line
            let inside_x = self.point1.x > rectangle_x1 && self.point1.x < rectangle_x2;
            let crosses_y = self.point1.y < rectangle_y1 && self.point2.y > rectangle_y2;
            // println!("\tinside_x: {inside_x}, crosses_y: {crosses_y}");
            inside_x && crosses_y
        } else {
            // Horizontal line
            // Swap points because line is now horizontal
            (rectangle_x1, rectangle_x2) = (rectangle_x2, rectangle_x1);
            (rectangle_y1, rectangle_y2) = (rectangle_y2, rectangle_y1);
            let inside_y = self.point1.y > rectangle_y1 && self.point1.y < rectangle_y2;
            let crosses_x = self.point1.x < rectangle_x1 && self.point2.x > rectangle_x2;
            // println!("\tinside_y: {inside_y}, crosses_x: {crosses_x}");
            inside_y && crosses_x
        }
    }
}

pub fn get_lines(points: &[Point]) -> Vec<Line> {
    points
        .windows(2)
        .map(|points| Line::new(points[0], points[1]))
        // Connect last and first
        .chain(iter::once(Line::new(points[points.len() - 1], points[0])))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_get_sorted_rectangles() {
        let input = include_str!("../input-test-part2.txt");
        let points = parse_points(input);
        let mut rectangles = get_sorted_rectangles(&points);
        println!("{}", rectangles.len());
        while let Some(rectangle) = rectangles.pop() {
            println!("{rectangle:?}");
        }
    }

    #[bench]
    fn bench_parse_points(b: &mut Bencher) {
        let input = include_str!("../input.txt");
        b.iter(|| parse_points(input));
    }

    #[bench]
    fn bench_get_lines(b: &mut Bencher) {
        let input = include_str!("../input.txt");
        let points = parse_points(input);
        b.iter(|| get_lines(&points));
    }

    #[bench]
    fn bench_get_sorted_areas(b: &mut Bencher) {
        let input = include_str!("../input.txt");
        let points = parse_points(input);
        b.iter(|| get_sorted_areas(&points));
    }

    #[bench]
    fn bench_get_sorted_rectangles(b: &mut Bencher) {
        let input = include_str!("../input.txt");
        let points = parse_points(input);
        b.iter(|| get_sorted_rectangles(&points));
    }
}
