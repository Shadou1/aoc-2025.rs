#![feature(test)]

extern crate test;

use template::part2;

fn main() {
    let input = include_str!("../../input.txt");
    let expected_result = 0;
    let result = part2::solution(input);
    assert!(expected_result == 0 || result == expected_result);
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
        let expected = todo!("Expected result");
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_input(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part2::solution(input));
    }
}
