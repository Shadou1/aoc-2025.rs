#![feature(test)]

extern crate test;

use reactor::part2;

fn main() {
    let input = include_str!("../../input.txt");
    let result = part2::solution_dfs(input);
    println!("{result:?}");
    let expected = 333657640517376;
    assert!(expected == 0 || result == expected);
}

#[cfg(test)]
mod tests {
    use super::*;
    use reactor::{dfs, parse_devices_vec};
    use test::Bencher;

    #[test]
    fn test_input() {
        let input = include_str!("../../input-test-part2.txt");
        let result = part2::solution_dfs(input);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_solution_dfs(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part2::solution_dfs(input));
    }

    #[bench]
    fn bench_solution_dfs_only(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        let outputs = parse_devices_vec(input);
        b.iter(|| {
            let mut counts = vec![u64::MAX; 26_usize.pow(3)];
            dfs(&outputs, &mut counts, part2::SVR, part2::FFT);
            dfs(&outputs, &mut counts, part2::FFT, part2::DAC);
            dfs(&outputs, &mut counts, part2::DAC, part2::OUT);
        });
    }
}
