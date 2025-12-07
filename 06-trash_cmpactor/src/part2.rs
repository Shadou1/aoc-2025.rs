pub fn solution(input: &str) -> u64 {
    let lines_count = input.lines().count();
    let numbers_lines: Vec<&[u8]> = input
        .lines()
        .take(lines_count - 1)
        .map(|line| line.as_bytes())
        .collect();
    let operators_line = input.lines().nth(lines_count - 1).unwrap().bytes();

    let mut sum: u64 = 0;
    let mut current_operator = b'+';
    let mut current_number_index = 0;
    let mut numbers: [u64; 4] = [0, 0, 0, 0];

    for (i, operator) in operators_line.enumerate() {
        if operator != b' ' {
            sum += match current_operator {
                b'*' => numbers.iter().map(|number| number.max(&1)).product::<u64>(),
                b'+' => numbers.iter().sum::<u64>(),
                _ => panic!("Unknown operator"),
            };
            current_operator = operator;
            current_number_index = 0;
            numbers.fill(0);
        }

        for digits in numbers_lines.iter() {
            if digits[i] == b' ' {
                continue;
            }
            numbers[current_number_index] =
                (numbers[current_number_index] * 10) + (digits[i] - 48) as u64;
        }
        current_number_index += 1;
    }

    // last operation
    sum += match current_operator {
        b'*' => numbers.iter().map(|number| number.max(&1)).product::<u64>(),
        b'+' => numbers.iter().sum::<u64>(),
        _ => panic!("Unknown operator"),
    };

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
}
