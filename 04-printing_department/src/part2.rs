use crate::{make_padded_grid, remove_paper};

pub fn solution(input: &str) -> u64 {
    let mut grid = make_padded_grid(input, 2);
    let mut total_removed = 0;
    let mut check_around = None;
    let mut removed_result;

    while {
        removed_result = remove_paper(&mut grid, 2, 4, check_around);
        removed_result.removed > 0
    } {
        total_removed += removed_result.removed;
        check_around = Some(&removed_result.removed_at);
    }

    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;
}
