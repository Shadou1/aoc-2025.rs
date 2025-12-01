pub mod part1;
pub mod part2;

pub fn parse_rotations(rotations: &str) -> impl Iterator<Item = i16> {
    rotations.lines().map(|line| match &line[0..1] {
        "L" => -line[1..].parse::<i16>().unwrap(),
        "R" => line[1..].parse::<i16>().unwrap(),
        _ => panic!("Unknown direction"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
}
