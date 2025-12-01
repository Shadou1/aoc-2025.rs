use crate::{Rotation::Left, Rotation::Right, parse_rotations};

pub fn solution(input: &str) -> u32 {
    let mut times_zero_angle: u32 = 0;
    let mut current_angle: i16 = 50;
    let rotations = parse_rotations(input);

    rotations.for_each(|(direction, angle)| {
        current_angle = (current_angle
            + match direction {
                Left => -angle,
                Right => angle,
            })
            % 100;
        if current_angle == 0 {
            times_zero_angle += 1;
        }
    });

    times_zero_angle
}

#[cfg(test)]
mod tests {
    use super::*;
}
