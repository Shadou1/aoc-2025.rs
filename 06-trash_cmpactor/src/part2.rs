pub fn solution(input: &str) -> u64 {
    let lines_count = input.lines().count();
    let numbers_lines: Vec<&[u8]> = input
        .lines()
        .take(lines_count - 1)
        .map(|line| line.as_bytes())
        .collect();
    let operators_line = input.lines().nth(lines_count - 1).unwrap();

    let mut sum: u64 = 0;
    let mut current_result = 0;
    let mut current_operator = b' ';
    for (i, operator) in operators_line.bytes().enumerate() {
        if operator != b' ' {
            // println!("{current_result}");
            sum += current_result;
            current_operator = operator;
            current_result = match current_operator {
                b'*' => 1,
                b'+' => 0,
                _ => panic!("Unknown operator"),
            };
        }

        let mut current_number = 0;
        for digits in numbers_lines.iter() {
            if digits[i] == b' ' {
                continue;
            }
            current_number = (current_number * 10) + (digits[i] - 48) as u64;
        }
        if current_number == 0 {
            continue
        }
        // println!("\t{i} : {current_number}");
        match current_operator {
            b'*' => current_result *= current_number,
            b'+' => current_result += current_number,
            _ => panic!("Unknown operator"),
        }
    }
    
    // last operation
    sum += current_result;

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
}
