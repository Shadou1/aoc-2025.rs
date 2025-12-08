use std::collections::HashSet;

use crate::parse_junction_boxes;

pub fn solution(input: &str) -> u64 {
    let boxes = parse_junction_boxes(input);
    let mut distances: Vec<((usize, usize), f64)> = Vec::with_capacity(boxes.len().pow(2));

    for box1_i in 0..boxes.len() {
        for box2_i in box1_i + 1..boxes.len() {
            distances.push(((box1_i, box2_i), boxes[box1_i].distnace(&boxes[box2_i])));
        }
    }

    distances.sort_by(|d1, d2| d1.1.total_cmp(&d2.1));

    let mut circuits: Vec<HashSet<usize>> = vec![HashSet::new()];
    circuits[0].insert(distances[0].0.0);
    circuits[0].insert(distances[0].0.1);
    for ((box1_i, box2_i), distance) in &distances[1..] {
        let mut circuit1_i: Option<usize> = None;
        let mut circuit2_i: Option<usize> = None;
        for (circuit_i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(box1_i) {
                circuit1_i = Some(circuit_i);
            } else if circuit.contains(box2_i) {
                circuit2_i = Some(circuit_i);
            }
            if circuit1_i.is_some() && circuit2_i.is_some() {
                break;
            }
        }

        match (circuit1_i, circuit2_i) {
            (None, None) => {
                // both boxes not part of some circuit
                let mut new_circuit = HashSet::new();
                new_circuit.insert(*box1_i);
                new_circuit.insert(*box2_i);
                circuits.push(new_circuit);
            }
            (Some(circuit1_i), None) => {
                // first box already in circuit
                circuits[circuit1_i].insert(*box2_i);
            }
            (None, Some(circuit2_i)) => {
                // second box already in circuit
                circuits[circuit2_i].insert(*box1_i);
            }
            (Some(circuit1_i), Some(circuit2_i)) => {
                // both boxes part of different citcuits
                // merge circuits
                let merged_circuit: HashSet<usize> = circuits[circuit1_i]
                    .iter()
                    .chain(circuits[circuit2_i].iter())
                    .copied()
                    .collect();
                if circuit1_i > circuit2_i {
                    circuits.swap_remove(circuit1_i);
                    circuits.swap_remove(circuit2_i);
                } else {
                    circuits.swap_remove(circuit2_i);
                    circuits.swap_remove(circuit1_i);
                }
                circuits.push(merged_circuit);
            }
        }

        if circuits[0].len() == boxes.len() {
            return (boxes[*box1_i].x * boxes[*box2_i].x) as u64;
        }
    }

    panic!("Couldn't connect all boxes");
}

#[cfg(test)]
mod tests {
    use super::*;

}
