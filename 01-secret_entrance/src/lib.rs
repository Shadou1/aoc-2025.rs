pub mod part1;
pub mod part2;

pub enum Rotation {
    Left,
    Right,
}

use Rotation::{Left, Right};

pub fn parse_rotations(rotations: &str) -> impl Iterator<Item = (Rotation, i16)> {
    rotations.lines().map(|line| {
        let (direction, angle) = line.split_at(1);
        (
            match direction {
                "L" => Left,
                "R" => Right,
                _ => panic!("Unknown direction"),
            },
            angle.parse::<i16>().unwrap(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
}
