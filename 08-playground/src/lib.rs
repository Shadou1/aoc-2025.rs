use std::collections::HashSet;

pub mod part1;
pub mod part2;

#[derive(Debug)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl<T: Iterator<Item = i64>> From<T> for JunctionBox {
    fn from(mut value: T) -> Self {
        Self {
            x: value.next().unwrap(),
            y: value.next().unwrap(),
            z: value.next().unwrap(),
        }
    }
}

impl JunctionBox {
    fn distnace(&self, other: &Self) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}

fn parse_junction_boxes(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .map(|line| JunctionBox::from(line.splitn(3, ',').map(|number| number.parse().unwrap())))
        .collect()
}

#[derive(Debug)]
struct Circuit {
    members: HashSet<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
