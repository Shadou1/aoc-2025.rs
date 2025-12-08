pub fn solution(input: &str) -> u64 {
    let mut lines = input.lines().map(|line| line.as_bytes());
    let first_line = lines.next().unwrap();
    let start = first_line.iter().position(|byte| *byte == b'S').unwrap();
    let mut beams = vec![0; first_line.len()];
    beams[start] = 1;

    for line in lines {
        for (i, &space) in line.iter().enumerate() {
            if space == b'^' && beams[i] > 0 {
                beams[i - 1] += beams[i];
                beams[i + 1] += beams[i];
                beams[i] = 0;
            }
        }
    }

    beams.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

}
