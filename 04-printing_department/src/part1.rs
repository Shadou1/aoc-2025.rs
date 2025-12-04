use std::iter;

use crate::{make_padded_grid, remove_paper};

pub fn solution(input: &str) -> u64 {
    let mut grid = make_padded_grid(input, 1);
    remove_paper(&mut grid, 1, 4, None).removed
}

#[cfg(test)]
mod tests {
    use super::*;
}
