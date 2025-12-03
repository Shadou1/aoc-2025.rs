use crate::largest_joltage;

pub fn solution(input: &str) -> u64 {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let bank: Vec<u8> = line.as_bytes().iter().map(|byte| byte - 48).collect();
        let largest_joltage = largest_joltage::<2>(&bank);
        // println!("Largest:\t{largest_joltage}");
        sum += largest_joltage;
    }

    sum
}

pub fn solution2(input: &str) -> u64 {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let bytes = line.as_bytes();
        let mut left = bytes[0];
        let mut right = bytes[1];

        for i in 1..line.len() - 1 {
            if bytes[i] > left {
                left = bytes[i];
                right = bytes[i + 1];
            } else if bytes[i] > right {
                right = bytes[i]
            }
        }
        if bytes[bytes.len() - 1] > right {
            right = bytes[bytes.len() - 1];
        }

        let largest_joltage = ((left - 48) * 10 + right - 48) as u64;
        // println!("{largest_joltage}");
        sum += largest_joltage;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
}
