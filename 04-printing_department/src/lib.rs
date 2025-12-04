use std::iter;

pub mod part1;
pub mod part2;

pub fn make_padded_grid(input: &str, padding: usize) -> Vec<Vec<u8>> {
    let line_len = input.lines().next().unwrap().len();
    let empty_padded_line: Vec<u8> = std::iter::repeat_n(b'.', line_len + padding * 2).collect();
    let mut padded_lines: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            iter::repeat_n(b'.', padding)
                .chain(line.bytes())
                .chain(iter::repeat_n(b'.', padding))
                .collect()
        })
        .collect();

    let mut grid = Vec::new();
    for _ in 0..padding {
        grid.push(empty_padded_line.clone());
    }
    grid.append(&mut padded_lines);
    for _ in 0..padding {
        grid.push(empty_padded_line.clone());
    }

    for line in grid.iter() {
        for c in line {
            // print!("{}", *c as char);
        }
        // println!();
    }
    // println!();

    grid
}

fn count_paper_at(grid: &Vec<Vec<u8>>, y: usize, x: usize) -> u8 {
    let mut paper_around = 0;
    for dy in -1_isize..=1 {
        let dy = (y as isize + dy) as usize;
        for dx in -1_isize..=1 {
            let dx = (x as isize + dx) as usize;
            paper_around += if grid[dy][dx] == b'@' { 1 } else { 0 }
        }
    }
    paper_around
}

pub struct RemoveResult {
    removed: u64,
    removed_at: Vec<(usize, usize)>,
}

pub fn remove_paper(
    grid: &mut Vec<Vec<u8>>,
    padding: usize,
    less_than: u8,
    check_around: Option<&[(usize, usize)]>,
) -> RemoveResult {
    let line_len = grid[0].len();
    let mut removed_at: Vec<(usize, usize)> = Vec::new();
    let mut removed = 0;

    if let Some(check_at) = check_around {
        for &(y, x) in check_at {
            // println!("At: {y}:{x}");
            for dy in -1_isize..=1 {
                let dy = (y as isize + dy) as usize;
                for dx in -1_isize..=1 {
                    let dx = (x as isize + dx) as usize;
                    if grid[dy][dx] != b'@' {
                        continue;
                    }
                    // print!("\tChecking: {dy}:{dx} - ");
                    let paper_around = count_paper_at(grid, dy, dx);
                    // print!("{paper_around} around - ");
                    if paper_around < less_than + 1 {
                        // println!("removing");
                        grid[dy][dx] = b'.';
                        removed_at.push((dy, dx));
                        removed += 1;
                    } else {
                        // println!("keeping");
                    }
                }
            }
        }
    } else {
        for y in padding..=grid.len() - (padding + 1) {
            for x in padding..=line_len - (padding + 1) {
                if grid[y][x] != b'@' {
                    continue;
                }
                let paper_around = count_paper_at(grid, y, x);
                if paper_around < less_than + 1 {
                    removed_at.push((y, x));
                    removed += 1;
                }
            }
        }

        for &(y, x) in removed_at.iter() {
            grid[y][x] = b'.';
        }
    }

    for line in grid.iter() {
        for c in line {
            // print!("{}", *c as char);
        }
        // println!();
    }
    // println!("Removed: {removed}");

    RemoveResult {
        removed,
        removed_at,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
