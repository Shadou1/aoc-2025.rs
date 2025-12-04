use crate::{make_padded_grid, remove_paper};

pub fn solution(input: &str) -> u64 {
    let mut grid = make_padded_grid(input);
    let mut total_removed = 0;
    let mut removed;

    while {
        removed = remove_paper(&mut grid, 4);
        removed > 0
    } {
        total_removed += removed;
    }

    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;
}
