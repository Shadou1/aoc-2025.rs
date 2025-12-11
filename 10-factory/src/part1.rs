use itertools::Itertools;

use crate::{BitFlag, LightMachine, parse_light_machines};

pub fn solution(input: &str) -> u64 {
    let mut sum = 0;
    let machines = parse_light_machines(input);
    'outer: for LightMachine { target, buttons } in machines {
        for i in 1..=buttons.len() {
            for buttons in buttons.iter().combinations(i) {
                let result = buttons.iter().fold(BitFlag(0), |acc, &&flag| acc ^ flag);
                if result == target {
                    sum += i;
                    continue 'outer;
                }
            }
        }
        panic!("Couldn't find a valid combination of button presses");
    }

    sum as u64
}

#[cfg(test)]
mod tests {
    use super::*;
}
