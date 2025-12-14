use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    ptr,
    thread::current,
    time::{self, SystemTime},
};

use crate::{ButtonMul, Constraint, Joltage, JoltageMachine, parse_joltage_machines};

pub fn solution(input: &str) -> u64 {
    let mut sum: u64 = 0;
    let mut machines = parse_joltage_machines(input);
    machines.sort_by_key(|machine| machine.target.levels.len());

    for JoltageMachine {
        mut target,
        buttons,
    } in machines
    {
        println!("===========\nNew Machine\n===========\n");
        println!("Target {target:?}");
        println!("Buttons {buttons:?}");

        // setup
        let mut length = 0;
        let mut buttons_refs: Vec<&Joltage> = buttons.iter().collect();
        let mut current: Joltage = Joltage {
            levels: vec![0; target.levels.len()],
        };
        let mut remaining: Joltage = target.clone();

        // build constraints on button presses
        let constraints = remaining.get_constraints(&buttons_refs);
        println!("Constraints: {constraints:?}");

        // satisfy constraints with a single button index
        for constraint in &constraints {
            if constraint.indexes.len() != 1 {
                continue;
            }

            let button_i_to_satisfy = constraint.indexes[0];
            let button = buttons_refs[button_i_to_satisfy];
            let mul = current.get_fitting_mul(button, &target);
            length += mul;
            // current = current.add_mul(button, mul);
            remaining = remaining.sub_mul(button, mul);
        }
        // remove satisfied buttons
        buttons_refs = buttons_refs
            .into_iter()
            .enumerate()
            .filter_map(|(button_i, button)| {
                if constraints.iter().any(|constraint| {
                    constraint.indexes.len() == 1 && constraint.indexes[0] == button_i
                }) {
                    None
                } else {
                    Some(button)
                }
            })
            .collect();

        // sort buttons
        buttons_refs.sort_unstable_by_key(|buttons| {
            buttons.levels.iter().filter(|&&level| level > 0).count()
        });
        buttons_refs.reverse();

        // rebuild constraints
        let constraints = remaining.get_constraints(&buttons_refs);
        // remove satisfied from target
        target = remaining.clone();

        println!("\nAfter simplifying");
        println!("Remaining {remaining:?}");
        println!("Buttons {buttons_refs:?}");
        println!("Constraints: {constraints:?}");

        if target == current {
            // problem simplifies to solution
            println!("Found configuration with length: {length} while simplifying\n");
            sum += length as u64;
            continue;
        }

        // setup
        let mut buttons_stack: Vec<ButtonMul> = Vec::new();
        let last_i_to_try = buttons_refs.len() - 1;
        let mut button_i_to_add = 0;
        // let mut last_i_when_subbing = 0;

        // steps
        let mut time_start = SystemTime::now();
        let mut steps_back: usize = 0;
        let mut steps_forward: usize = 0;

        'check: while current != target {
            // println!();

            // figure out if completing target is possible

            // check remaining buttons can complete target
            // let remaining_buttons = &buttons_refs[ref_i_to_add..];
            // if !remaining.can_complete_with_buttons(remaining_buttons) {
            //     println!("Can't complete buttons using starting index {ref_i_to_add}");
            //
            //     steps_back += 1;
            //     let last_stack_button = buttons_stack.last_mut().expect("Stack is empty");
            //     let last_button = buttons_refs[last_stack_button.index];
            //     current = current.sub_mul(last_button, 1);
            //     remaining = remaining.add_mul(last_button, 1);
            //     last_stack_button.mul -= 1;
            //
            //     ref_i_to_add = last_stack_button.index + 1;
            //
            //     if last_stack_button.mul == 0 {
            //         println!("Removed last");
            //         buttons_stack.pop();
            //     }
            //
            //     // current = current.sub_mul(last_button, last_stack_button.mul);
            //     // remaining = remaining.add_mul(last_button, last_stack_button.mul);
            //     // buttons_stack.pop();
            //
            //     println!("Stack: {buttons_stack:?}");
            //     println!("Current: {current:?}");
            //     println!("Remaining {remaining:?}");
            //     continue;
            // }

            // // check next button press will overflow target
            // let mut increased_i = false;
            // while ref_i_to_add < buttons_refs.len()
            //     && current.will_be_larger_than_target(buttons_refs[ref_i_to_add], &target)
            // {
            //     increased_i = true;
            //     ref_i_to_add += 1;
            //     println!("Increase ref_i_to_add to {ref_i_to_add}");
            // }
            // if increased_i {
            //     continue;
            // }

            // get next fitting button index
            let ref_i_that_fits =
                remaining.get_next_fitting_button(&buttons_refs[button_i_to_add..]);
            button_i_to_add += ref_i_that_fits.unwrap_or(last_i_to_try + 1);

            // couldn't find a button to push from this buttons_stack combination
            if button_i_to_add > last_i_to_try {
                // pop last added buttons from stack
                if let Some(ButtonMul { index, mul }) = buttons_stack.pop() {
                    steps_back += 1;
                    let button = buttons_refs[index];
                    current = current.sub_mul(button, mul);
                    remaining = remaining.add_mul(button, mul);
                } else {
                    panic!("Stack is empty");
                };

                // walk back one more button
                steps_back += 1;
                let last_stack_button = buttons_stack.last_mut().expect("Stack is empty");
                let last_button = buttons_refs[last_stack_button.index];
                last_stack_button.mul -= 1;
                current = current.sub_mul(last_button, 1);
                remaining = remaining.add_mul(last_button, 1);
                let first_i_to_try = last_stack_button.index + 1;
                if last_stack_button.mul == 0 {
                    // No more mul in the last buttons stack
                    buttons_stack.pop();
                }

                // println!("Popping last stack + walking back 1");
                // println!("Stack: {buttons_stack:?}");
                // println!("Current: {current:?}");
                // println!("Remaining {remaining:?}");

                // find button that fits starting from last_popped_i (excluding it)
                // for i in first_i_to_try..=last_i_to_try {
                //     if !current.will_be_larger_than_target(buttons_refs[i], &target) {
                //         ref_i_to_add = i;
                //         break;
                //     }
                // }
                let ref_i_that_fits =
                    remaining.get_next_fitting_button(&buttons_refs[first_i_to_try..]);
                button_i_to_add = first_i_to_try + ref_i_that_fits.unwrap_or(last_i_to_try + 1);
            }

            // check if constraint is broken
            // let broken_constraint = constraints
            //     .iter()
            //     .find(|constraint| !constraint.holds(&buttons_stack));
            // if let Some(broken_constraint) = broken_constraint {
            //     println!("BROKEN");
            //     // TODO remove/walk back mul of broken constraint
            //     // walk back one
            //     steps_back += 1;
            //     let last_stack_button = buttons_stack
            //         .last_mut()
            //         .expect("Stack is empty when constraint is broken");
            //     let last_button = buttons_refs[last_stack_button.0];
            //     last_stack_button.1 -= 1;
            //     current = current.sub_mul(last_button, 1);
            //     remaining = remaining.add_mul(last_button, 1);
            //     let first_i_to_try = last_stack_button.0 + 1;
            //     if last_stack_button.1 == 0 {
            //         // No more mul in the last buttons stack
            //         buttons_stack.pop();
            //     }
            //
            //     // println!("Constraint is broken: {broken_constraint:?}");
            //     // println!("Subtracting 1 mul");
            //     // println!("New Stack: {:?}", buttons_stack);
            //     // println!("New Current: {:?}", current);
            //
            //     // find button that fits starting from last_popped_i (excluding it)
            //     // println!("Searching for buttons from i = {last_stack_button.0}");
            //     for i in first_i_to_try..=last_i_to_try {
            //         if !current.will_be_larger_than_target(buttons_refs[i], &target) {
            //             ref_i_to_add = i;
            //             break;
            //         }
            //     }
            //
            //     continue;
            // }

            // steps_forward += 1;
            // // last_i_when_subbing = ref_i_to_add;
            // let button_to_add = buttons_refs[ref_i_to_add];
            // let mul = current.get_fitting_mul(button_to_add, &target);
            // buttons_stack.push(ButtonMul {
            //     index: ref_i_to_add,
            //     mul,
            // });
            // current = current.add_mul(button_to_add, mul);
            // remaining = remaining.sub_mul(button_to_add, mul);
            // ref_i_to_add += 1;

            // add largest buttons until nothing fits
            while button_i_to_add <= last_i_to_try {
                // if only 1 button left to try
                if button_i_to_add == last_i_to_try && !buttons_stack.is_empty() {
                    // if ref_i_to_add == last_i_to_try {
                    // try to complete target with last button
                    // println!("Checking max mul of last button");
                    let last_possible_button = buttons_refs[button_i_to_add];
                    let mul = current.get_fitting_mul(last_possible_button, &target);
                    let result = current.clone().add_mul(last_possible_button, mul);
                    if result != target {
                        // didn't match target, need to walk back
                        // walk back one button
                        steps_back += 1;
                        let last_stack_button = buttons_stack.last_mut().expect("Stack is empty");
                        let last_button = buttons_refs[last_stack_button.index];

                        if button_i_to_add == last_stack_button.index + 1 {
                            // only last button to try, can remove a multiple
                            // let result = current.clone();
                            // for mul_to_sub in 1..last_stack_button.mul {
                            //     result.sub_mul(button, mul)
                            // }
                            // println!("Subtracting {} mul", last_stack_button.mul);
                            current = current.sub_mul(last_button, last_stack_button.mul);
                            remaining = remaining.add_mul(last_button, last_stack_button.mul);
                            last_stack_button.mul = 0;
                        } else {
                            // other buttons can fit, can remove only 1
                            // println!("Subtracting 1 mul");
                            current = current.sub_mul(last_button, 1);
                            remaining = remaining.add_mul(last_button, 1);
                            last_stack_button.mul -= 1;
                            // BAD
                            // current = current.sub_mul(last_button, last_stack_button.mul);
                            // remaining = remaining.add_mul(last_button, last_stack_button.mul);
                            // last_stack_button.mul = 0;
                        }

                        button_i_to_add = last_stack_button.index + 1;
                        // if last_stack_button.index != last_i_when_subbing {
                        //     ref_i_to_add = last_stack_button.index + 1;
                        //     println!("Setting ref_i_to_add to {ref_i_to_add}");
                        // }
                        if last_stack_button.mul == 0 {
                            // println!("Removed last");
                            // No more mul in the last buttons stack
                            // let first_i_to_try = last_stack_button.index;
                            // for i in first_i_to_try..=last_i_to_try {
                            //     if !current.will_be_larger_than_target(buttons_refs[i], &target) {
                            //         ref_i_to_add = i;
                            //         break;
                            //     }
                            // }
                            buttons_stack.pop();
                        }
                        // TEST pop last stack
                        // steps_back += 1;
                        // if let Some((button_i, mul)) = buttons_stack.pop() {
                        //     steps_back += 1;
                        //     let button = buttons_refs[button_i];
                        //     current = current.sub_mul(button, mul);
                        //     remaining = remaining.add_mul(button, mul);
                        // } else {
                        //     panic!("Stack is empty");
                        // };

                        // println!("Stack: {buttons_stack:?}");
                        // println!("Current: {current:?}");
                        // println!("Remaining {remaining:?}");

                        continue;
                    }
                }

                // check remaining buttons can complete target
                // walk back mul times
                if let Some(last_stack_button) = buttons_stack.last()
                    && button_i_to_add == last_stack_button.index + 1
                {
                    // checking all possible buttons is slower
                    // let possible_buttons = &buttons_refs[last_stack_button.index + 1..];
                    let remaining_buttons = &buttons_refs[button_i_to_add..];
                    let last_button = buttons_refs[last_stack_button.index];
                    let mul_to_sub = remaining.get_mul_to_sub(remaining_buttons, last_button);
                    match mul_to_sub {
                        crate::StackMul::Max => {
                            // println!("Cannot complete target by going back");
                            // steps_back += 1;
                            // ref_i_to_add = last_stack_button.index + 1;
                            //
                            // current = current.sub_mul(last_button, last_stack_button.mul);
                            // remaining = remaining.add_mul(last_button, last_stack_button.mul);
                            // buttons_stack.pop();
                            //
                            // println!("Stack: {buttons_stack:?}");
                            // println!("Current: {current:?}");
                            // println!("Remaining {remaining:?}");

                            // continue;
                        }
                        crate::StackMul::Some(mut mul_to_sub) => {
                            // println!(
                            //     "Cannot complete target by going forward, backing {mul_to_sub}"
                            // );
                            steps_back += 1;
                            button_i_to_add = last_stack_button.index + 1;

                            if mul_to_sub > last_stack_button.mul {
                                // println!(
                                //     "Attempt to remove {mul_to_sub} mul, more than current mul"
                                // );
                                mul_to_sub = last_stack_button.mul;
                            }

                            let mut last_stack_button =
                                buttons_stack.last_mut().expect("Stack is empty");
                            let last_button = buttons_refs[last_stack_button.index];
                            current = current.sub_mul(last_button, mul_to_sub);
                            remaining = remaining.add_mul(last_button, mul_to_sub);
                            last_stack_button.mul -= mul_to_sub;

                            // while last_stack_button.mul == 0 {
                            //     println!("Removed last");
                            //     buttons_stack.pop();
                            //     last_stack_button =
                            //         buttons_stack.last_mut().expect("Stack is empty");
                            //     let last_button = buttons_refs[last_stack_button.index];
                            //     current = current.sub_mul(last_button, 1);
                            //     remaining = remaining.add_mul(last_button, 1);
                            //     last_stack_button.mul -= 1;
                            //     ref_i_to_add -= 1;
                            // }

                            if last_stack_button.mul == 0 {
                                button_i_to_add = last_stack_button.index + 1;
                                buttons_stack.pop();
                            }

                            // while last_stack_button.mul == 0 {
                            //     // if mul is 0, pop the stack and remove one more button
                            //     // keep removing if mul is 0 again
                            //     steps_back += 1;
                            //     buttons_stack.pop();
                            //
                            //     if buttons_stack.is_empty() {
                            //         break;
                            //     }
                            //
                            //     last_stack_button = buttons_stack.last_mut().unwrap();
                            //     let last_button = buttons_refs[last_stack_button.index];
                            //     current = current.sub_mul(last_button, 1);
                            //     remaining = remaining.add_mul(last_button, 1);
                            //     last_stack_button.mul -= 1;
                            //
                            //     ref_i_to_add = last_stack_button.index + 1;
                            //     // if last_stack_button.mul == 0 {
                            //     //     println!("Removed last twice");
                            //     //     buttons_stack.pop();
                            //     //     let last_stack_button =
                            //     //         buttons_stack.last().expect("Stack is empty");
                            //     //     ref_i_to_add = last_stack_button.index + 1;
                            //     //     // ref_i_to_add -= 1;
                            //     // } else {
                            //     //     println!("Removed last");
                            //     //     // ref_i_to_add -= 1;
                            //     // }
                            // }

                            // println!("Stack: {buttons_stack:?}");
                            // println!("Current: {current:?}");
                            // println!("Remaining {remaining:?}");

                            // continue;
                            continue 'check;
                        }
                        crate::StackMul::None => {
                            // println!("Could complete target by going forward");
                        }
                    }
                }

                let button_to_add = buttons_refs[button_i_to_add];
                let mul = remaining.get_remaining_mul(buttons_refs[button_i_to_add]);
                if mul == 0 {
                    button_i_to_add += 1;
                    continue;
                }

                steps_forward += 1;
                buttons_stack.push(ButtonMul {
                    index: button_i_to_add,
                    mul,
                });
                current = current.add_mul(button_to_add, mul);
                remaining = remaining.sub_mul(button_to_add, mul);

                // println!("Adding: {:?}", buttons_stack[buttons_stack.len() - 1]);
                // println!("Stack: {buttons_stack:?}");
                // println!("Current: {current:?}");
                // println!("Remaining: {remaining:?}");

                button_i_to_add += 1;
            }

            // println!("Setting last_i_when_subbing to {ref_i_to_add}");
            // println!("Stack: {buttons_stack:?}");
            // println!("Current: {current:?}");
            // println!("Remaining: {remaining:?}");

            // let smallest_target_indexes = remaining
            //     .levels
            //     .iter()
            //     .enumerate()
            //     .fold(
            //         (u16::MAX, vec![]),
            //         |(min, mut min_indexes), (target_i, target)| {
            //             if *target == 0 {
            //                 return (min, min_indexes);
            //             }
            //             match target.cmp(&min) {
            //                 Ordering::Less => (*target, vec![target_i]),
            //                 Ordering::Equal => {
            //                     min_indexes.push(target_i);
            //                     (min, min_indexes)
            //                 }
            //                 Ordering::Greater => (min, min_indexes),
            //             }
            //         },
            //     )
            //     .1;
            //
            // if smallest_target_indexes.is_empty() {
            //     panic!("All targets already satisfied")
            // }
            //
            // println!("Smallest targets: {smallest_target_indexes:?}");
            //
            // let buttons_to_try: Vec<&Joltage> = buttons_refs
            //     .iter()
            //     .filter(|button| {
            //         smallest_target_indexes
            //             .iter()
            //             .any(|target_i| button.levels[*target_i] == 1)
            //     })
            //     .copied()
            //     .collect();
            //
            // println!("Buttons to try {buttons_to_try:?}");
            //
            // match buttons_to_try.len() {
            //     1 => {
            //         // Only one way to hit this target
            //         let button = buttons_to_try[0];
            //         let mul = current.get_mul(button, &target);
            //         buttons_stack.push((mul, button));
            //         current = current.add_mul(button, mul);
            //         remaining = remaining.sub_mul(button, mul);
            //     }
            //     2 => todo!(),
            //     _ => todo!(),
            // };
        }

        length += buttons_stack
            .iter()
            .fold(0, |sum, ButtonMul { mul, .. }| sum + mul);
        println!(
            "Found configuration with length: {length} in {steps_forward} steps forward, {steps_back} steps back, total {} steps",
            steps_forward + steps_back
        );
        println!("Final stack: {buttons_stack:?}");
        println!(
            "Took {} seconds",
            SystemTime::now()
                .duration_since(time_start)
                .unwrap()
                .as_secs_f32()
        );
        println!();
        sum += length as u64;
    }

    sum
}

pub fn solution_brute_force(input: &str) -> u64 {
    let mut sum = 0;
    let mut machines = parse_joltage_machines(input);
    machines.sort_by_key(|machine| machine.target.levels.len());

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
        let mut buttons_stack: Vec<(u16, &Joltage)> = Vec::new();
        println!("Target {target:?}");
        println!("Buttons {buttons_ref:?}");
        let mut ref_i_to_add = 0;
        // let mut prev_added_i: Vec<usize> = Vec::with_capacity(buttons_stack.len());
        // let mut last_button_i = 0;
        let mut last_i_to_try = buttons_ref.len();
        // let mut buttons_to_check = &buttons_ref[ref_i..];
        let mut checked_joltages: HashMap<usize, HashSet<Joltage>> = HashMap::new();
        let mut result: Joltage = Joltage {
            levels: vec![0; target.levels.len()],
        };

        let mut skipped: usize = 0;
        let mut steps: usize = 0;

        while result != target {
            while result.will_be_larger_than_target(buttons_ref[ref_i_to_add], &target) {
                ref_i_to_add += 1;

                if ref_i_to_add >= last_i_to_try {
                    // couldn't find a button to push from this buttons_stack combination
                    // pop last added buttons from stack
                    if let Some((mul, last_button)) = buttons_stack.pop() {
                        steps += 1;
                        // println!("Popping {last_button:?} {mul} times");
                        result = result.sub_mul(last_button, mul);
                    } else {
                        panic!("Stack is empty");
                    };

                    let checked = checked_joltages.entry(buttons_stack.len()).or_default();
                    checked.insert(result.clone());

                    // remove one more
                    steps += 1;
                    let last_button_stack = buttons_stack.last_mut().expect("Stack is empty");
                    last_button_stack.0 -= 1;
                    let mut last_button_i = buttons_ref
                        .iter()
                        .position(|r| ptr::eq(*r, last_button_stack.1))
                        .expect("Couldn't find the index of the last popped button");
                    result = result.sub_mul(last_button_stack.1, 1);
                    if last_button_stack.0 == 0 {
                        // No more mul in the last buttons stack
                        let checked = checked_joltages.entry(buttons_stack.len()).or_default();
                        checked.insert(result.clone());
                        buttons_stack.pop();
                    }

                    // if !buttons_stack.is_empty() {
                    //     checked_joltages.insert(result.clone());
                    // }

                    // println!("Popping last stack");
                    // println!("Subtracting 1 mul");
                    // println!("New Stack: {:?}", buttons_stack);
                    // println!("New Result: {:?}", result);

                    // find button that fits starting from last_popped_i (excluding it)
                    // println!("Searching for buttons from i = {last_button_i}");
                    for i in last_button_i + 1..=last_i_to_try {
                        if !result.will_be_larger_than_target(buttons_ref[i], &target) {
                            ref_i_to_add = i;
                            break;
                        }
                    }

                    continue;
                }

                continue;
            }

            // println!("Result is: {result:?}");
            // println!("Checked is: {checked_joltages:?}");
            'index: while let Some(checked_at_index) = checked_joltages.get(&buttons_stack.len()) {
                while checked_at_index.contains(&result) {
                    skipped += 1;
                    // println!("Not checking {:?} again", result);
                    // remove one mul from stack
                    let last_button_stack = buttons_stack.last_mut().expect("Stack is empty");
                    last_button_stack.0 -= 1;
                    let last_button_i = buttons_ref
                        .iter()
                        .position(|r| ptr::eq(*r, last_button_stack.1))
                        .expect("Couldn't find the index of the last popped button");
                    result = result.sub_mul(last_button_stack.1, 1);
                    if last_button_stack.0 == 0 {
                        // No more mul in the last buttons stack
                        buttons_stack.pop();
                        continue 'index;
                    }

                    for i in last_button_i + 1..=last_i_to_try {
                        if !result.will_be_larger_than_target(buttons_ref[i], &target) {
                            ref_i_to_add = i;
                            break;
                        }
                    }
                }
                break;
            }

            // find mul of buttons_ref[ref_i_to_add] that won't overflow
            // println!("Before add: {:?}", result);

            steps += 1;
            let button_ref_to_add = buttons_ref[ref_i_to_add];
            let mul = result.get_fitting_mul(button_ref_to_add, &target);
            buttons_stack.push((mul, button_ref_to_add));
            result = result.add_mul(button_ref_to_add, mul);

            // println!();
            // println!("Adding: {:?}", buttons_stack[buttons_stack.len() - 1]);
            // println!("Stack: {:?}", buttons_stack);
            // println!("Result: {}", result);
        }

        let result = buttons_stack.iter().fold(0, |sum, (mul, _)| sum + mul);
        println!(
            "Found configuration with length: {result} in {steps} steps, skipped {skipped} times\n",
        );
        sum += result as u64;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
}
