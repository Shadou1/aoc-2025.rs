use std::iter;

pub fn solution(input: &str) -> u64 {
    let numbers_count = input.lines().count() - 1;
    let problems_count = input.lines().next().unwrap().split_whitespace().count();

    let mut numbers: Vec<Vec<u64>> =
        iter::repeat_n(Vec::with_capacity(numbers_count - 1), problems_count).collect();
    for (i, number) in input.lines().take(numbers_count).flat_map(|line| {
        line.split_whitespace()
            .map(|number| number.parse::<u64>().unwrap())
            .enumerate()
    }) {
        numbers[i].push(number);
    }

    let mut sum: u64 = 0;
    for operator in input.lines().nth(numbers_count).unwrap().split_whitespace() {
        sum += numbers
            .remove(0)
            .into_iter()
            .reduce(|number1, number2| match operator {
                "+" => number1 + number2,
                "*" => number1 * number2,
                _ => panic!("Unknown operator"),
            })
            .unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
}
