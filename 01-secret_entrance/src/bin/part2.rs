#![feature(test)]

extern crate test;

use secret_entrance::part2;

fn main() {
    let input = include_str!("../../input.txt");
    let result = part2::solution(input);
    println!("{result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_input() {
        let input = include_str!("../../input-test-part2.txt");
        let result = part2::solution(input);
        let expected = 6;
        assert_eq!(result, expected);
    }

    #[test]
    fn extra_test_1() {
        let input = include_str!("../../input-test.txt");
        let result = part2::solution(input);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_input(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part2::solution(input));
    }
}
