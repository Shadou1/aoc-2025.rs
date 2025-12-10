#![feature(test)]

extern crate test;

use movie_theater::part2;

fn main() {
    let input = include_str!("../../input.txt");
    let result = part2::solution(input);
    println!("{result:?}");
    let expected = 1542119040;
    assert!(expected == 0 || result == expected);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_input() {
        let input = include_str!("../../input-test-part2.txt");
        let result = part2::solution(input);
        let expected = 24;
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part2::solution(input));
    }
}
