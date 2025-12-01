use crate::{Rotation::Left, Rotation::Right, parse_rotations};

pub fn solution(input: &str) -> u32 {
    let mut times_zero_angle: u32 = 0;
    let mut current_angle: i16 = 50;
    let rotations = parse_rotations(input);

    rotations.for_each(|(direction, angle)| {
        print!("Before : {current_angle},\t");
        let prev_angle = current_angle;
        current_angle += match direction {
            Left => -angle,
            Right => angle,
        };
        print!("After : {current_angle},\t");

        if current_angle < 0 && prev_angle != 0 || current_angle == 0 {
            times_zero_angle += 1;
        }
        times_zero_angle += (current_angle / 100).abs() as u32;
        current_angle = (current_angle % 100 + 100) % 100;
        // current_angle.rem_euclid(100);
        print!("Wrapped : {current_angle}.\t");
        println!("Passed zero: {times_zero_angle} times.");
    });

    times_zero_angle
}

#[cfg(test)]
mod tests {
    use super::*;
}
