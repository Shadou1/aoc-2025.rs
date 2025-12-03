use crate::largest_joltage;

pub fn solution(input: &str) -> u64 {
    let mut sum = 0;

    for line in input.lines() {
        let bank: Vec<u8> = line.as_bytes().iter().map(|byte| byte - 48).collect();
        let largest_joltage = largest_joltage::<12>(&bank);
        println!("Largest:\t{largest_joltage}");
        sum += largest_joltage;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
}
