#![feature(test)]

use std::ops::BitXor;
extern crate test;

pub mod part1;
pub mod part2;

#[derive(PartialEq, Eq, Clone, Copy)]
struct BitFlag(u64);

impl<T: Iterator<Item = u8>> From<T> for BitFlag {
    fn from(value: T) -> Self {
        Self(value.fold(0, |acc, byte| acc + (1 << byte)))
    }
}

impl BitXor for BitFlag {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitFlag(self.0 ^ rhs.0)
    }
}

pub struct Machine {
    target: BitFlag,
    buttons: Vec<BitFlag>,
}

pub fn parse_machine(bytes: &[u8]) -> Machine {
    let target_end_i = bytes.iter().position(|byte| *byte == b']').unwrap();
    let target: BitFlag = bytes[1..target_end_i]
        .iter()
        .enumerate()
        .filter_map(|(i, byte)| match byte {
            b'#' => Some(i as u8),
            b'.' => None,
            _ => panic!("Unknown byte when parsing target"),
        })
        .into();

    let buttons_start_i = bytes.iter().position(|byte| *byte == b'(').unwrap();
    let joltage_start_i = bytes.iter().position(|byte| *byte == b'{').unwrap();
    let buttons: Vec<BitFlag> = bytes[buttons_start_i..joltage_start_i - 1]
        .split(|byte| *byte == b' ')
        .map(|bytes| {
            bytes
                .iter()
                .filter_map(|byte| match byte {
                    b'0'..=b'9' => Some(byte - 48),
                    b'(' | b')' | b',' => None,
                    _ => panic!("Unknown byte when parsing button"),
                })
                .into()
        })
        .collect();

    Machine { target, buttons }
}

pub fn parse_machines(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| line.as_bytes())
        .map(parse_machine)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_parse_machine() {
        let input = include_str!("../input-test-part1.txt");
        let machine = parse_machine(input.lines().nth(1).unwrap().as_bytes());
        println!("{:08b}", machine.target.0);
        for button in &machine.buttons {
            println!("{:08b}", button.0);
        }
    }

    #[bench]
    fn bench_parse_machines(b: &mut Bencher) {
        let input = include_str!("../input.txt");
        b.iter(|| parse_machines(input));
    }
}
