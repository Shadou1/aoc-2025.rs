use std::ptr;

use crate::{Joltage, JoltageMachine, parse_joltage_machines};

pub fn solution(input: &str) -> u64 {
    let mut sum = 0;
    let machines = parse_joltage_machines(input);

    for JoltageMachine {
        target,
        mut buttons,
    } in machines
    {
        buttons.sort_unstable_by_key(|buttons| {
            buttons.levels.iter().filter(|&&level| level > 0).count()
        });
        buttons.reverse();
        let buttons_ref: Vec<&Joltage> = buttons.iter().collect();
        let mut buttons_stack: Vec<&Joltage> = Vec::new();
        println!("Buttons {buttons_ref:?}");
        let mut ref_i_to_add = 0;
        // let mut prev_added_i: Vec<usize> = Vec::with_capacity(buttons_stack.len());
        let mut last_popped_i = 0;
        let mut last_i_to_try = buttons_ref.len();
        // let mut buttons_to_check = &buttons_ref[ref_i..];
        let mut result: Joltage = Joltage {
            levels: vec![0; target.levels.len()],
        };

        let mut steps: usize = 0;

        while result != target {
            while result.will_be_larger_than_target(buttons_ref[ref_i_to_add], &target) {
                ref_i_to_add += 1;

                if ref_i_to_add >= last_i_to_try {
                    // couldn't find a button to push from this buttons_stack combination
                    while let Some(last) = buttons_stack.pop() {
                        steps += 1;
                        println!("Popping {last:?}");
                        // pop same buttons from stack
                        result = result - last;
                        if !ptr::eq(last, buttons_stack[buttons_stack.len() - 1]) {
                            break;
                        }
                    }
                    // pop one more
                    steps += 1;
                    let last = buttons_stack
                        .pop()
                        .expect("Button stack is empty and cannot find button to add");
                    last_popped_i = buttons_ref
                        .iter()
                        .position(|r| ptr::eq(*r, last))
                        .expect("Couldn't find the index of the last popped button");
                    println!("Last pop {:?}", last);
                    println!("Last pop i {}", last_popped_i);

                    result = result - last;
                    println!("Stack: {:?}", buttons_stack);
                    println!("Result {:?}", result);

                    // find button that fits starting from last_popped_i (excluding it)
                    println!("Searching for buttons from i = {last_popped_i}");
                    for i in last_popped_i + 1..=last_i_to_try {
                        if !result.will_be_larger_than_target(buttons_ref[i], &target) {
                            ref_i_to_add = i;
                            break;
                        }
                    }

                    continue;
                }

                continue;
            }

            steps += 1;
            println!();
            buttons_stack.push(buttons_ref[ref_i_to_add]);
            println!("Adding {:?}", buttons_ref[ref_i_to_add]);
            result = result + buttons_stack[buttons_stack.len() - 1];
            println!("Stack: {:?}", buttons_stack);
            println!("Result: {:?}", result);
        }

        println!(
            "Found configuration with length: {} in {steps} steps",
            buttons_stack.len()
        );
        sum += buttons_stack.len() as u64;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
}
