pub fn solution(input: &str) -> u64 {
    let mut lines = input.lines().map(|line| line.as_bytes());
    let first_line = lines.next().unwrap();
    let start = first_line.iter().position(|byte| *byte == b'S').unwrap();
    let mut beams = vec![false; first_line.len()];
    beams[start] = true;

    let mut times_split = 0;
    for line in lines {
        for (i, &space) in line.iter().enumerate() {
            if space == b'^' && beams[i] {
                times_split += 1;
                beams[i - 1] = true;
                beams[i + 1] = true;
                beams[i] = false;
            }
        }
    }

    times_split
}

#[cfg(test)]
mod tests {
    use super::*;
}
