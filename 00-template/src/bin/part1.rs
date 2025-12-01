#![feature(test)]

extern crate test;

use template::part1;

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
        let expected = todo!("Expected result");
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_input(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part1::solution(input));
    }
}
