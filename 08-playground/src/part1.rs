use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::{
    BoxDistance, DisjointSet, get_sorted_distances, get_sorted_distances_binary_heap,
    parse_junction_boxes,
};

pub fn connect_boxes_map_box_to_circuit(
    distances: &[BoxDistance],
    boxes_count: usize,
    connections_count: usize,
) -> u64 {
    let mut circuits: Vec<Vec<usize>> = vec![Vec::with_capacity(boxes_count / 2)];
    circuits[0].push(distances[0].box1_i);
    circuits[0].push(distances[0].box2_i);
    let mut box_to_circuit: HashMap<usize, usize> = HashMap::with_capacity(1000);
    box_to_circuit.insert(distances[0].box1_i, 0);
    box_to_circuit.insert(distances[0].box2_i, 0);

    for &BoxDistance { box1_i, box2_i, .. } in &distances[1..connections_count] {
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
    }

    circuits.sort_by_key(|circuit| std::cmp::Reverse(circuit.len()));
    circuits
        .into_iter()
        .map(|circuit| circuit.len())
        .take(3)
        .product::<usize>() as u64
}

pub fn solution(input: &str, connections: usize) -> u64 {
    let boxes = parse_junction_boxes(input);
    let boxes_len = boxes.len();
    let distances = get_sorted_distances(&boxes);

    connect_boxes_map_box_to_circuit(&distances, boxes_len, connections)
}

pub fn connect_boxes_disjoint_set(
    distances: &[BoxDistance],
    boxes_count: usize,
    connections_count: usize,
) -> u64 {
    let mut circuits = DisjointSet::new(boxes_count);
    for &BoxDistance { box1_i, box2_i, .. } in &distances[..connections_count] {
        circuits.union(box1_i, box2_i);
    }
    circuits.sizes.sort();
    circuits.sizes.reverse();
    circuits.sizes.into_iter().take(3).product::<usize>() as u64
}

pub fn solution_disjoint_set(input: &str, connections: usize) -> u64 {
    let boxes = parse_junction_boxes(input);
    let boxes_len = boxes.len();
    let distances = get_sorted_distances(&boxes);

    connect_boxes_disjoint_set(&distances, boxes_len, connections)
}

pub fn connect_boxes_disjoint_set_binary_heap(
    mut distances: BinaryHeap<Reverse<BoxDistance>>,
    boxes_count: usize,
    connections_count: usize,
) -> u64 {
    let mut circuits = DisjointSet::new(boxes_count);
    for _ in 0..connections_count {
        let BoxDistance { box1_i, box2_i, .. } = distances.pop().unwrap().0;
        circuits.union(box1_i, box2_i);
    }
    circuits.sizes.sort();
    circuits.sizes.reverse();
    circuits.sizes.into_iter().take(3).product::<usize>() as u64
}

pub fn solution_disjoint_set_binary_heap(input: &str, connections: usize) -> u64 {
    let boxes = parse_junction_boxes(input);
    let boxes_len = boxes.len();
    let distances = get_sorted_distances_binary_heap(&boxes);

    connect_boxes_disjoint_set_binary_heap(distances, boxes_len, connections)
}

#[cfg(test)]
mod tests {
    use super::*;
}
