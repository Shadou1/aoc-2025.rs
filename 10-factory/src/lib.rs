#![feature(test)]

use std::{
    cmp::Ordering,
    ops::{Add, BitXor, Sub},
};
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

impl PartialOrd for BitFlag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BitFlag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

pub struct LightMachine {
    target: BitFlag,
    buttons: Vec<BitFlag>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Joltage {
    levels: Vec<u16>,
}

impl<T: Iterator<Item = u16>> From<T> for Joltage {
    fn from(value: T) -> Self {
        Self {
            levels: value.collect(),
        }
    }
}

impl Add for Joltage {
    type Output = Joltage;

    fn add(self, rhs: Self) -> Self::Output {
        <Self as Add<&Joltage>>::add(self, &rhs)
    }
}

impl Add<&Joltage> for Joltage {
    type Output = Joltage;

    fn add(mut self, rhs: &Joltage) -> Self::Output {
        for (level, level_rhs) in self.levels.iter_mut().zip(rhs.levels.iter()) {
            *level += level_rhs
        }
        self
    }
}

impl Sub for Joltage {
    type Output = Joltage;

    fn sub(self, rhs: Self) -> Self::Output {
        <Self as Sub<&Joltage>>::sub(self, &rhs)
    }
}

impl Sub<&Joltage> for Joltage {
    type Output = Joltage;

    fn sub(mut self, rhs: &Joltage) -> Self::Output {
        for (level, level_rhs) in self.levels.iter_mut().zip(rhs.levels.iter()) {
            *level -= level_rhs
        }
        self
    }
}

impl Joltage {
    fn will_be_larger_than_target(&self, button: &Joltage, target: &Joltage) -> bool {
        for ((level, button), &target) in self
            .levels
            .iter()
            .zip(button.levels.iter())
            .zip(target.levels.iter())
        {
            if level + button > target {
                return true;
            }
        }

        false
    }
}

pub struct JoltageMachine {
    target: Joltage,
    buttons: Vec<Joltage>,
}

fn parse_lights(bytes: &[u8]) -> BitFlag {
    let target_end_i = bytes.iter().position(|byte| *byte == b']').unwrap();
    bytes[1..target_end_i]
        .iter()
        .enumerate()
        .filter_map(|(i, byte)| match byte {
            b'#' => Some(i as u8),
            b'.' => None,
            _ => panic!("Unknown byte when parsing ligths"),
        })
        .into()
}

fn parse_buttons(bytes: &[u8]) -> Vec<BitFlag> {
    let buttons_start_i = bytes.iter().position(|byte| *byte == b'(').unwrap();
    let joltage_start_i = bytes.iter().position(|byte| *byte == b'{').unwrap();
    bytes[buttons_start_i..joltage_start_i - 1]
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
        .collect()
}

fn parse_joltage(bytes: &[u8]) -> Joltage {
    let joltage_start_i = bytes.iter().position(|byte| *byte == b'{').unwrap();
    let joltage_end_i = bytes.iter().position(|byte| *byte == b'}').unwrap();
    bytes[joltage_start_i + 1..joltage_end_i]
        .split(|byte| *byte == b',')
        .map(|bytes| {
            bytes
                .iter()
                .fold(0_u16, |acc, byte| acc * 10 + (*byte as u16 - 48))
        })
        .into()
}

fn parse_buttons_joltage(bytes: &[u8]) -> Vec<Joltage> {
    let buttons_start_i = bytes.iter().position(|byte| *byte == b'(').unwrap();
    let joltage_start_i = bytes.iter().position(|byte| *byte == b'{').unwrap();
    let mut joltages = Vec::with_capacity(10);

    for levels in bytes[buttons_start_i..joltage_start_i - 1].split(|byte| *byte == b' ') {
        let mut joltage_levels = vec![0_u16; 10];
        for level in levels {
            match level {
                b'0'..=b'9' => joltage_levels[(level - 48) as usize] = 1,
                b'(' | b')' | b',' => (),
                _ => panic!("Unknown byte when parsing joltage"),
            }
        }
        joltages.push(Joltage {
            levels: joltage_levels,
        })
    }

    joltages
}

pub fn parse_light_machine(bytes: &[u8]) -> LightMachine {
    let target = parse_lights(bytes);
    let buttons = parse_buttons(bytes);

    LightMachine { target, buttons }
}

pub fn parse_joltage_machine(bytes: &[u8]) -> JoltageMachine {
    let target = parse_joltage(bytes);
    let buttons = parse_buttons_joltage(bytes);

    JoltageMachine { target, buttons }
}

pub fn parse_light_machines(input: &str) -> Vec<LightMachine> {
    input
        .lines()
        .map(|line| line.as_bytes())
        .map(parse_light_machine)
        .collect()
}

pub fn parse_joltage_machines(input: &str) -> Vec<JoltageMachine> {
    input
        .lines()
        .map(|line| line.as_bytes())
        .map(parse_joltage_machine)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_parse_light_machine() {
        let input = include_str!("../input-test-part1.txt");
        let machine = parse_light_machine(input.lines().nth(1).unwrap().as_bytes());
        println!("{:08b}", machine.target.0);
        for button in &machine.buttons {
            println!("{:08b}", button.0);
        }
    }

    #[test]
    fn test_parse_joltage_machine() {
        let input = include_str!("../input-test-part1.txt");
        let machine = parse_joltage_machine(input.lines().nth(1).unwrap().as_bytes());
        println!("{:?}", machine.target.levels);
        for button in &machine.buttons {
            println!("{:?}", button.levels);
        }
    }

    #[bench]
    fn bench_parse_light_machines(b: &mut Bencher) {
        let input = include_str!("../input.txt");
        b.iter(|| parse_light_machines(input));
    }

    #[bench]
    fn bench_parse_joltage_machines(b: &mut Bencher) {
        let input = include_str!("../input.txt");
        b.iter(|| parse_joltage_machines(input));
    }
}
