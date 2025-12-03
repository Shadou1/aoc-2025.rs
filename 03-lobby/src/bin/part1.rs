#![feature(test)]

extern crate test;

use lobby::part1;

fn main() {
    let input = include_str!("../../input.txt");
    let result = part1::solution(input);
    println!("{result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_input() {
        let input = include_str!("../../input-test-part1.txt");
        let result = part1::solution(input);
        let expected = 357;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_input_zig() {
        let input = include_str!("../../input-test-part1.txt");
        let result = part1::solution_zig(input);
        let expected = 357;
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_input(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part1::solution(input));
    }

    #[bench]
    fn bench_input_zig(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part1::solution_zig(input));
    }
}
