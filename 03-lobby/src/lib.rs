pub mod part1;
pub mod part2;

pub fn largest_joltage<const L: usize>(bank: &[u8]) -> u64 {
    let mut batteries: [u8; L] = [0; L];
    batteries[..L].copy_from_slice(&bank[..L]);
    println!("Start:\t\t{:?},", batteries);

    for bank_i in 1..bank.len() - L + 1 {
        for battery_i in 0..L {
            if bank[bank_i + battery_i] > batteries[battery_i] {
                batteries[battery_i..L]
                    .copy_from_slice(&bank[bank_i + battery_i..bank_i + battery_i + L - battery_i]);
                println!("New best:\t{:?},", batteries);
                break;
            }
        }
    }

    for bank_i in bank.len() - L + 1..bank.len() {
        let battery_i = batteries.len() - (bank.len() - bank_i);
        // dbg!(batteries.len(), bank.len(), bank_i);
        if bank[bank_i] > batteries[battery_i] {
            batteries[battery_i..].copy_from_slice(&bank[bank_i..]);
            println!("Last best:\t{:?},", batteries);
            break;
        }
    }

    let mut joltage: u64 = 0;
    for battery_i in 0..L {
        joltage += batteries[battery_i] as u64 * 10u64.pow((L - battery_i - 1) as u32);
    }
    joltage
}

#[cfg(test)]
mod tests {
    use super::*;
}
