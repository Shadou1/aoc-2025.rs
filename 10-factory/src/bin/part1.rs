#![feature(test)]

extern crate test;

use factory::part1;

fn main() {
    let input = include_str!("../../input.txt");
    let result = part1::solution(input);
    println!("{result:?}");
    let expected = 461;
    assert!(expected == 0 || result == expected);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_input() {
        let input = include_str!("../../input-test-part1.txt");
        let result = part1::solution(input);
        let expected = 7;
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part1::solution(input));
    }
}
