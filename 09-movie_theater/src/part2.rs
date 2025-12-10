extern crate test;

use std::collections::BinaryHeap;

use crate::{Line, Rectangle, get_lines, get_sorted_rectangles, parse_points};

pub fn solution(input: &str) -> u64 {
    let points = parse_points(input);
    let rectangles = get_sorted_rectangles(&points);
    let lines = get_lines(&points);

    get_biggest_rectangle_area_without_lines_inside(rectangles, &lines)
}

pub fn get_biggest_rectangle_area_without_lines_inside(
    mut rectangles: BinaryHeap<Rectangle>,
    lines: &[Line],
) -> u64 {
    'outer: while let Some(rectangle) = rectangles.pop() {
        // println!("{}", rectangle.0);
        for line in lines {
            // println!("\t{}", line);
            if line.is_inside(&rectangle) {
                // println!("\tFound line inside");
                continue 'outer;
            }
        }
        // Found biggest rectangle without any lines inside
        // println!("Found biggest area");
        // println!("{}", rectangle.0);
        return rectangle.area();
    }

    panic!("Couldn't find largest rectangle without any lines in it");
}

#[cfg(test)]
mod tests {
    use super::*;
}
