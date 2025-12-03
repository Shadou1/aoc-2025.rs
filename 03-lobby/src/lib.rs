pub mod part1;
pub mod part2;

pub fn largest_joltage<const L: usize>(bank: &[u8]) -> u64 {
    let mut batteries: [u8; L] = [0; L];
    batteries[..L].copy_from_slice(&bank[..L]);
    // println!("Start:\t\t{:?},", batteries);

    for bank_i in 1..bank.len() - L + 1 {
        for battery_i in 0..L {
            let bank_i_to_check = bank_i + battery_i;
            if bank[bank_i_to_check] > batteries[battery_i] {
                batteries[battery_i..L].copy_from_slice(&bank[bank_i_to_check..bank_i + L]);
                // println!("New best:\t{:?},", batteries);
                break;
            }
        }
    }

    let mut joltage: u64 = 0;
    for battery_i in 0..L {
        joltage += batteries[battery_i] as u64 * 10u64.pow((L - battery_i - 1) as u32);
    }
    joltage
}

pub fn largest_joltage_zig(bank: &[u8], length: usize) -> u64 {
    let mut joltage = String::with_capacity(length);
    let mut search_start_i = 0;
    let search_slice_len = bank.len() - length;
    let mut search_end_i = search_slice_len;

    while joltage.len() < length {
        // dbg!(search_start_i, search_end_i);
        let (mut largest_battery_i, mut largest_battery) = (search_start_i, bank[search_start_i]);
        for i in search_start_i + 1..=search_end_i {
            if bank[i] > largest_battery {
                largest_battery_i = i;
                largest_battery = bank[i];
            }
        }
        joltage.push(largest_battery as char);
        search_start_i = largest_battery_i + 1;
        search_end_i = search_start_i + bank.len() - (length - joltage.len()) - search_start_i;
        // dbg!(&joltage);
    }

    joltage.parse().expect("Malformed number")
}

#[cfg(test)]
mod tests {
    use super::*;
}
