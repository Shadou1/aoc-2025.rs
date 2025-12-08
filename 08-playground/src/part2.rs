use std::collections::{HashMap, HashSet};

use crate::parse_junction_boxes;

pub fn solution(input: &str) -> u64 {
    let boxes = parse_junction_boxes(input);
    let mut distances: Vec<((usize, usize), u64)> = Vec::with_capacity(boxes.len().pow(2));

    for box1_i in 0..boxes.len() {
        for box2_i in box1_i + 1..boxes.len() {
            distances.push(((box1_i, box2_i), boxes[box1_i].distnace_squared(&boxes[box2_i])));
        }
    }

    distances.sort_by(|d1, d2| d1.1.cmp(&d2.1));

    let mut circuits: Vec<Vec<usize>> = vec![Vec::with_capacity(10)];
    circuits[0].push(distances[0].0.0);
    circuits[0].push(distances[0].0.1);
    let mut box_to_circuit: HashMap<usize, usize> = HashMap::with_capacity(1000);
    box_to_circuit.insert(distances[0].0.0, 0);
    box_to_circuit.insert(distances[0].0.1, 0);

    for &((box1_i, box2_i), _) in &distances[1..] {
        // print!("\nBoxes {box1_i} : {box2_i}\t");
        let circuit1_i = box_to_circuit.get(&box1_i);
        let circuit2_i = box_to_circuit.get(&box2_i);

        match (circuit1_i, circuit2_i) {
            (None, None) => {
                // both boxes not part of some circuit
                let new_circuit = vec![box1_i, box2_i];
                // println!("New:\t{:?}", new_circuit);
                circuits.push(new_circuit);
                box_to_circuit.insert(box1_i, circuits.len() - 1);
                box_to_circuit.insert(box2_i, circuits.len() - 1);
            }
            (Some(&circuit1_i), None) => {
                // first box already in circuit
                circuits[circuit1_i].push(box2_i);
                box_to_circuit.insert(box2_i, circuit1_i);
                // println!("Add second:\t{:?}", circuits[circuit1_i]);
            }
            (None, Some(&circuit2_i)) => {
                // second box already in circuit
                circuits[circuit2_i].push(box1_i);
                box_to_circuit.insert(box1_i, circuit2_i);
                // println!("Add first:\t{:?}", circuits[circuit2_i]);
            }
            (Some(&circuit1_i), Some(&circuit2_i)) if circuit1_i != circuit2_i => {
                // both boxes part of different circuits
                // merge circuits
                let mut merged_circuit: Vec<usize> =
                    Vec::with_capacity(circuits[circuit1_i].len() + circuits[circuit2_i].len());
                for box1_i in &circuits[circuit1_i] {
                    box_to_circuit.insert(*box1_i, circuits.len());
                    merged_circuit.push(*box1_i);
                }
                for box2_i in &circuits[circuit2_i] {
                    box_to_circuit.insert(*box2_i, circuits.len());
                    merged_circuit.push(*box2_i);
                }
                circuits[circuit1_i].clear();
                circuits[circuit2_i].clear();
                // println!("Merge:\t{:?}", merged_circuit);
                circuits.push(merged_circuit);
            }
            (Some(_), Some(_)) => {
                // both boxes part of the same circuit
                // println!("In same circuit:\tskip");
            }
        }
        // println!("Circuits:\t{:?}", circuits);
        // println!("Map:\t{:?}", box_to_circuit);
        if circuits[circuits.len() - 1].len() == boxes.len() {
            return (boxes[box1_i].x * boxes[box2_i].x) as u64;
        }
    }

    panic!("Couldn't connect all boxes");
}

#[cfg(test)]
mod tests {
    use super::*;

}
