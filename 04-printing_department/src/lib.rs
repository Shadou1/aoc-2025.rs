use std::iter;

pub mod part1;
pub mod part2;

pub fn make_padded_grid(input: &str) -> Vec<Vec<u8>> {
    let line_len = input.lines().next().unwrap().len();
    let empty_padded_line: Vec<u8> = std::iter::repeat_n(b'.', line_len + 2).collect();
    let mut padded_lines: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            iter::once(b'.')
                .chain(line.bytes())
                .chain(iter::once(b'.'))
                .collect()
        })
        .collect();
    let mut grid = Vec::new();
    grid.push(empty_padded_line.clone());
    grid.append(&mut padded_lines);
    grid.push(empty_padded_line);
    // for line in grid.iter() {
    //     for c in line {
    //         print!("{}", *c as char);
    //     }
    //     println!()
    // }

    grid
}

pub fn remove_paper(grid: &mut Vec<Vec<u8>>, less_than: u8) -> u64 {
    let line_len = grid[0].len();
    let mut to_remove: Vec<(usize, usize)> = Vec::new();
    let mut removed = 0;

    for y in 1..=grid.len() - 2 {
        for x in 1..=line_len - 2 {
            if grid[y][x] == b'.' {
                continue;
            }
            let mut paper_around = 0;
            // print!("@ At {y}:{x}: ");
            for dy in -1_isize..=1 {
                for dx in -1_isize..=1 {
                    paper_around +=
                        if grid[(y as isize + dy) as usize][(x as isize + dx) as usize] == b'@' {
                            1
                        } else {
                            0
                        }
                }
            }
            // println!("{paper_around} paper");
            if paper_around < less_than + 1 {
                to_remove.push((y, x));
                removed += 1;
            }
        }
    }

    for (y, x) in to_remove {
        grid[y][x] = b'.';
    }

    removed
}

#[cfg(test)]
mod tests {
    use super::*;
}
