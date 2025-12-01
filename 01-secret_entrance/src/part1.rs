use crate::parse_rotations;

pub fn solution(input: &str) -> u32 {
    let mut times_at_zero: u32 = 0;
    let mut current_angle: i16 = 50;

    parse_rotations(input).for_each(|rotation| {
        current_angle = (current_angle + rotation) % 100;
        if current_angle == 0 {
            times_at_zero += 1;
        }
    });

    times_at_zero
}

#[cfg(test)]
mod tests {
    use super::*;
}
