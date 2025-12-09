#![feature(test)]

extern crate test;

use playground::part1;

fn main() {
    let input = include_str!("../../input.txt");
    let expected_result = 24360;
    let result = part1::solution_disjoint_set_binary_heap(input, 1000);
    println!("{result:?}");
    assert_eq!(result, expected_result);
    let result = part1::solution(input, 1000);
    println!("{result:?}");
    assert_eq!(result, expected_result);
    let result = part1::solution_disjoint_set(input, 1000);
    println!("{result:?}");
    assert_eq!(result, expected_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use playground::{
        get_sorted_distances, get_sorted_distances_binary_heap, parse_junction_boxes,
    };
    use test::Bencher;

    #[test]
    fn test_input() {
        let input = include_str!("../../input-test-part1.txt");
        let result = part1::solution(input, 10);
        let expected = 40;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_disjoint_set() {
        let input = include_str!("../../input-test-part1.txt");
        let result = part1::solution_disjoint_set(input, 10);
        let expected = 40;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_disjoint_set_binary_heap() {
        let input = include_str!("../../input-test-part1.txt");
        let result = part1::solution_disjoint_set_binary_heap(input, 10);
        let expected = 40;
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_box_to_circuit(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part1::solution(input, 1000));
    }

    #[bench]
    fn bench_disjoint_set(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part1::solution_disjoint_set(input, 1000));
    }

    #[bench]
    fn bench_disjoint_set_binary_heap(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| part1::solution_disjoint_set_binary_heap(input, 1000));
    }

    #[bench]
    fn _bench_build_data_structure(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| {
            let boxes = parse_junction_boxes(input);
            let distances = get_sorted_distances(&boxes);
        });
    }

    #[bench]
    fn _bench_build_data_structure_binary_heap(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        b.iter(|| {
            let boxes = parse_junction_boxes(input);
            let distances = get_sorted_distances_binary_heap(&boxes);
        });
    }

    #[bench]
    fn bench_box_to_circuit_without_input(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        let boxes = parse_junction_boxes(input);
        let boxes_len = boxes.len();
        let distances = get_sorted_distances(&boxes);
        b.iter(|| part1::connect_boxes_map_box_to_circuit(&distances, boxes_len, 1000));
    }

    #[bench]
    fn bench_disjoint_set_without_input(b: &mut Bencher) {
        let input = include_str!("../../input.txt");
        let boxes = parse_junction_boxes(input);
        let boxes_len = boxes.len();
        let distances = get_sorted_distances(&boxes);
        b.iter(|| part1::connect_boxes_disjoint_set(&distances, boxes_len, 1000));
    }
}
