#![feature(test)]
extern crate test;

use std::{
    cmp::Ordering,
    fmt,
    hash::Hash,
    iter,
    ops::{Add, BitXor, Mul, Sub},
};

use itertools::Itertools;

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Joltage {
    levels: Vec<u16>,
}

#[derive(PartialEq, Eq, Debug)]
struct Constraint {
    indexes: Vec<usize>,
    sum: u16,
}

#[derive(PartialEq, Eq)]
struct ButtonMul {
    index: usize,
    mul: u16,
}

impl Constraint {
    pub fn holds(&self, buttons_stack: &[ButtonMul]) -> bool {
        let mut sum = 0;
        for ButtonMul { index, mul } in buttons_stack {
            for constraint_i in &self.indexes {
                if index == constraint_i {
                    sum += mul;
                    if sum > self.sum {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl fmt::Debug for ButtonMul {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ i: {}, mul: {} }}", self.index, self.mul)
    }
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

impl Hash for Joltage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.levels.hash(state);
    }
}

impl fmt::Debug for Joltage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.levels)
    }
}

impl fmt::Display for Joltage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.levels)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum StackMul {
    Max,
    Some(u16),
    None,
}

impl Ord for StackMul {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (StackMul::Max, StackMul::Max) => Ordering::Equal,
            (StackMul::Max, StackMul::Some(_)) => Ordering::Greater,
            (StackMul::Max, StackMul::None) => Ordering::Greater,
            (StackMul::Some(_), StackMul::Max) => Ordering::Less,
            (StackMul::Some(mul1), StackMul::Some(mul2)) => mul1.cmp(mul2),
            (StackMul::Some(_), StackMul::None) => Ordering::Greater,
            (StackMul::None, StackMul::Max) => Ordering::Less,
            (StackMul::None, StackMul::Some(_)) => Ordering::Less,
            (StackMul::None, StackMul::None) => Ordering::Equal,
        }
    }
}

impl PartialOrd for StackMul {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for StackMul {
    type Output = StackMul;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (StackMul::Max, _) | (_, StackMul::Max) => StackMul::Max,
            (StackMul::Some(mul1), StackMul::Some(mul2)) => StackMul::Some(mul1 + mul2),
            (StackMul::Some(mul), StackMul::None) => StackMul::Some(mul),
            (StackMul::None, StackMul::Some(mul)) => StackMul::Some(mul),
            (StackMul::None, StackMul::None) => StackMul::None,
        }
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

    /// call on remaining targets
    fn get_next_fitting_button(&self, buttons: &[&Joltage]) -> Option<usize> {
        for (button_i, button) in buttons.iter().enumerate() {
            if !self
                .levels
                .iter()
                .zip(button.levels.iter())
                .any(|(remaining, button)| *remaining == 0 && *button == 1)
            {
                return Some(button_i);
            }
        }
        None
    }

    fn get_constraints(&self, buttons: &[&Joltage]) -> Vec<Constraint> {
        self.levels
            .iter()
            .enumerate()
            .filter_map(|(target_i, target)| {
                if *target == 0 {
                    None
                } else {
                    Some(Constraint {
                        indexes: buttons
                            .iter()
                            .enumerate()
                            .filter_map(|(button_i, button)| {
                                if button.levels[target_i] == 1 {
                                    Some(button_i)
                                } else {
                                    None
                                }
                            })
                            .collect(),
                        sum: *target,
                    })
                }
            })
            .unique_by(|target| target.indexes.clone())
            .collect()
    }

    fn get_fitting_mul(&self, button: &Joltage, target: &Joltage) -> u16 {
        target
            .levels
            .iter()
            .zip(self.levels.iter())
            .zip(button.levels.iter())
            .filter_map(|((target, current), button)| match button {
                1 => Some(target - current),
                0 => None,
                _ => panic!("Button has value other than 0 / 1"),
            })
            .min()
            .unwrap()
    }

    /// call on remaining targets
    fn get_remaining_mul(&self, button: &Joltage) -> u16 {
        self.levels
            .iter()
            .zip(button.levels.iter())
            .filter_map(|(remaining, button)| match button {
                1 => Some(*remaining),
                0 => None,
                _ => panic!("Button has value other than 0 / 1"),
            })
            .min()
            .unwrap()
    }

    /// call on remaining targets
    fn get_mul_to_sub(&self, buttons: &[&Joltage], last_stack_button: &Joltage) -> StackMul {
        println!("Finding max safe mul to back");
        println!("Buttons: {buttons:?}, Last: {last_stack_button:?}");

        let buttons_max_muls: Vec<u16> = buttons
            .iter()
            .map(|button| {
                self.levels
                    .iter()
                    .zip(button.levels.iter())
                    .filter_map(|(target, button)| match button {
                        1 => Some(*target),
                        0 => None,
                        _ => panic!("Button has value other than 0 / 1"),
                    })
                    .min()
                    .unwrap()
            })
            .collect();

        // INCORRECT, must take current/remaining target into consideration
        // let buttons_mul_increase_step: u16 = buttons
        //     .iter()
        //     .zip(iter::repeat_n(last_stack_button, buttons.len()))
        //     .map(|(button, last_button)| {
        //         for i in 0..button.levels.len() {
        //             match (button.levels[i], last_button.levels[i]) {
        //                 (1, 0) => return 0,
        //                 _ => continue,
        //             }
        //         }
        //         1
        //     })
        //     .sum();

        let max_presses = buttons_max_muls.iter().copied().sum::<u16>();
        let least_presses_needed = *self.levels.iter().max().unwrap();
        let mut remaining_presses = least_presses_needed.saturating_sub(max_presses);

        if remaining_presses == 0 {
            println!("No presses remaining");
            // might complete remaining target with current buttons
            return StackMul::None;
        }

        let mut max_muls_per_button: Vec<StackMul> = buttons
            .iter()
            .map(|button| {
                let mut min_mul = StackMul::Max;
                for ((target, button), last_button) in self
                    .levels
                    .iter()
                    .zip(button.levels.iter())
                    .zip(last_stack_button.levels.iter())
                {
                    min_mul = min_mul.min(match (target, button, last_button) {
                        (1.., 1, 0) => StackMul::Some(*target),
                        (_, 1, 1) => StackMul::Max,
                        (0, 1, 0) => StackMul::None,
                        _ => min_mul,
                    });
                }
                min_mul
            })
            .collect();

        println!(
            "Max presses: {:?}, Least needed: {:?}",
            max_presses, least_presses_needed
        );
        println!("Remaining presses: {:?}", remaining_presses);
        println!("Max mul per button {:?}", max_muls_per_button);

        let mut total_safe_steps_to_take = StackMul::None;
        while remaining_presses > 0 {
            let (positive_mul_count, positive_mul_min) = max_muls_per_button.iter().fold(
                (0, StackMul::Max),
                |(acc_count, acc_mul), &current_mul| {
                    let new_mul = match (acc_mul, current_mul) {
                        (StackMul::Some(acc_mul), StackMul::Some(current_mul)) => {
                            if current_mul < acc_mul {
                                StackMul::Some(current_mul)
                            } else {
                                StackMul::Some(acc_mul)
                            }
                        }
                        (_, StackMul::Some(current_mul)) => StackMul::Some(current_mul),
                        _ => acc_mul,
                    };
                    let new_count = acc_count
                        + if matches!(current_mul, StackMul::None) {
                            0
                        } else {
                            1
                        };
                    (new_count, new_mul)
                },
            );
            if positive_mul_count == 0 {
                return total_safe_steps_to_take;
            }

            let unsafe_steps_to_take = StackMul::Some(remaining_presses / positive_mul_count);
            println!("Unsafe steps: {:?}", unsafe_steps_to_take);
            if let StackMul::Some(mul) = unsafe_steps_to_take
                && mul == 0
            {
                return total_safe_steps_to_take;
            }

            let safe_steps_to_take = unsafe_steps_to_take.min(positive_mul_min);
            println!("Safe steps: {:?}", safe_steps_to_take);
            total_safe_steps_to_take = total_safe_steps_to_take + safe_steps_to_take;

            // take safe steps back
            if let StackMul::Some(steps) = total_safe_steps_to_take {
                remaining_presses = remaining_presses.saturating_sub(steps * positive_mul_count);
            } else {
                panic!();
            }
            // update max button muls
            max_muls_per_button.iter_mut().for_each(|mut mul| {
                if let (StackMul::Some(min_mul), StackMul::Some(curr_mul)) =
                    (positive_mul_min, &mut mul)
                {
                    *curr_mul = curr_mul.saturating_sub(min_mul);
                    if *curr_mul == 0 {
                        *mul = StackMul::None;
                    }
                }
            });

            println!("Max mul per button {:?}", max_muls_per_button);
            println!("Total safe steps: {:?}", total_safe_steps_to_take);
            println!("Remaining presses: {:?}", remaining_presses);
        }

        return total_safe_steps_to_take;

        // WRONG LOGIC
        // if max_presses >= least_presses_needed || safe_mul_to_back == 0 {
        //     return StackMul::None;
        // } else {
        //     return StackMul::Some(1);
        // }

        // println!("Mul step {:?}", buttons_mul_increase_step);
        //
        // if buttons_mul_increase_step == 0 {
        //     // last + remaining buttons cannot increase target
        //     return StackMul::Max;
        // }
        //
        // if max_presses >= least_presses_needed {
        //     return StackMul::None;
        // }
        //
        // let mul_to_sub = (least_presses_needed - max_presses) / buttons_mul_increase_step;
        // let mul_to_sub_safe = mul_to_sub.min(max_presses);
        // if mul_to_sub > 0 {
        //     return StackMul::Some(mul_to_sub_safe);
        // } else {
        //     return StackMul::None;
        // }

        // let mul_to_sub = (max_presses - least_presses_needed) / buttons_mul_increase_step;
        //
        // println!("Mul to sub {:?}", mul_to_sub);

        // if mul_to_sub > 0 {
        //     Some(mul_to_sub)
        // } else {
        //     None
        // }
    }

    /// call on remaining targets
    fn can_complete_with_buttons(&self, buttons: &[&Joltage]) -> bool {
        if buttons.len() != 2 {
            return true;
        }

        let buttons_max_muls: Vec<u16> = buttons
            .iter()
            .map(|button| {
                self.levels
                    .iter()
                    .zip(button.levels.iter())
                    .filter_map(|(target, button)| match button {
                        1 => Some(*target),
                        0 => None,
                        _ => panic!("Button has value other than 0 / 1"),
                    })
                    .min()
                    .unwrap()
            })
            .collect();
        let least_presses_needed = *self.levels.iter().max().unwrap();
        // println!("max muls: {buttons_max_muls:?}");
        // println!("least presses needed: {least_presses_needed:?}");

        if buttons_max_muls.iter().copied().sum::<u16>() < least_presses_needed {
            // even by pressin all buttons a maximum number of times, cannot complete remaining
            return false;
        }

        // if buttons.len() == 2 {
        //     println!("Looking at button combination of length {least_presses_needed}")
        //
        // }

        true

        // iter::repeat_n(buttons[0], button_1_max_mul)
    }

    fn add_mul(mut self, button: &Joltage, mul: u16) -> Self {
        for i in 0..self.levels.len() {
            self.levels[i] += button.levels[i] * mul;
        }
        self
    }

    fn sub_mul(mut self, button: &Joltage, mul: u16) -> Self {
        // println!("Subbing {mul} of {button:?} from {self:?}");
        for i in 0..self.levels.len() {
            self.levels[i] -= button.levels[i] * mul;
        }
        self
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
