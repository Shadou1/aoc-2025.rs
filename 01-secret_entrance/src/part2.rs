use crate::parse_rotations;

pub fn solution(input: &str) -> u32 {
    let mut times_passed_zero: u32 = 0;
    let mut current_angle: i16 = 50;

    parse_rotations(input).for_each(|rotation| {
        let prev_angle = current_angle;
        current_angle += rotation;

        if current_angle < 0 && prev_angle != 0 || current_angle == 0 {
            times_passed_zero += 1;
        }
        times_passed_zero += u32::from((current_angle / 100).unsigned_abs());

        current_angle = (current_angle % 100 + 100) % 100;
        // current_angle.rem_euclid(100);
    });

    times_passed_zero
}

#[cfg(test)]
mod tests {
    use super::*;
}
