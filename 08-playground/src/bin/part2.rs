#![feature(test)]

extern crate test;

use playground::part2;

fn main() {
    let input = include_str!("../../input.txt");
    let expected_result = 2185817796;
    let result = part2::solution_disjoint_set_binary_heap(input);
    println!("{result:?}");
    assert_eq!(result, expected_result);
    let result = part2::solution(input);
    println!("{result:?}");
    assert_eq!(result, expected_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_input() {
        let input = include_str!("../../input-test-part2.txt");
        let result = part2::solution(input);
        let expected = 25272;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_disjoint_set_binary_heap() {
        let input = include_str!("../../input-test-part1.txt");
        let result = part2::solution_disjoint_set_binary_heap(input);
        let expected = 25272;
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_input(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part2::solution(input));
    }

    #[bench]
    fn bench_disjoint_set_binary_heap(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part2::solution_disjoint_set_binary_heap(input));
    }
}
