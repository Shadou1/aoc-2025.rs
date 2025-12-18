#![feature(test)]

extern crate test;

use reactor::part1;

fn main() {
    let input = include_str!("../../input.txt");
    let result = part1::solution_dfs(input);
    println!("{result:?}");
    let expected = 423;
    assert!(expected == 0 || result == expected);
}

#[cfg(test)]
mod tests {
    use super::*;
    use reactor::{
        OUT, YOU, dfs, parse_devices_hashmap, parse_devices_vec,
        part1::{count_out_hashmap, count_out_vec},
    };
    use test::Bencher;

    #[test]
    fn test_input_hashmap() {
        let input = include_str!("../../input-test-part1.txt");
        let result = part1::solution_hashmap(input);
        let expected = 5;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_input_vec() {
        let input = include_str!("../../input-test-part1.txt");
        let result = part1::solution_vec(input);
        let expected = 5;
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_solution_hashmap(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part1::solution_hashmap(input));
    }

    #[bench]
    fn bench_parse_hashmap(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| parse_devices_hashmap(input));
    }

    #[bench]
    fn bench_only_solution_hashmap(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        let devices = parse_devices_hashmap(input);
        b.iter(|| count_out_hashmap(&devices))
    }

    #[bench]
    fn bench_solution_vec(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part1::solution_vec(input));
    }

    #[bench]
    fn bench_parse_vec(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| parse_devices_vec(input));
    }

    #[bench]
    fn bench_only_solution_vec(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        let devices = parse_devices_vec(input);
        b.iter(|| count_out_vec(&devices))
    }

    #[bench]
    fn bench_solution_dfs(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part1::solution_dfs(input))
    }

    #[bench]
    fn bench_only_solution_dfs(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        let devices = parse_devices_vec(input);
        b.iter(|| {
            let mut counts = vec![u64::MAX; 26_usize.pow(3)];
            dfs(&devices, &mut counts, YOU, OUT)
        });
    }
}
