#![feature(test)]

extern crate test;

use movie_theater::part1;

fn main() {
    let input = include_str!("../../input.txt");
    let result = part1::solution(input);
    let expected = 4777824480;
    assert!(expected == 0 || result == expected);
    println!("{result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use movie_theater::{get_sorted_areas, parse_points, part1::get_largest_area};
    use test::Bencher;

    #[test]
    fn test_input() {
        let input = include_str!("../../input-test-part1.txt");
        let result = part1::solution(input);
        let expected = 50;
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_input(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part1::solution(input));
    }

    #[bench]
    fn bench_parse_points(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| parse_points(input));
    }

    #[bench]
    fn bench_get_sorted_areas(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        let points = parse_points(input);
        b.iter(|| get_sorted_areas(&points));
    }

    #[bench]
    fn bench_only_solution(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        let points = parse_points(input);
        let areas = get_sorted_areas(&points);
        b.iter(|| get_largest_area(&areas));
    }
}
