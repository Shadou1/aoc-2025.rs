use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
};

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl<T: Iterator<Item = i64>> From<T> for JunctionBox {
    fn from(mut value: T) -> Self {
        Self {
            x: value.next().unwrap(),
            y: value.next().unwrap(),
            z: value.next().unwrap(),
        }
    }
}

impl JunctionBox {
    fn distnace(&self, other: &Self) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
    fn distnace_squared(&self, other: &Self) -> u64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as u64
    }
}

// pub fn parse_junction_boxes(input: &str) -> Vec<JunctionBox> {
//     input
//         .lines()
//         .map(|line| JunctionBox::from(line.splitn(3, ',').map(|number| number.parse().unwrap())))
//         .collect()
// }

pub fn parse_junction_boxes(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .map(|line| {
            JunctionBox::from(
                line.as_bytes()
                    .splitn(3, |&byte| byte == b',')
                    .map(|number| {
                        number
                            .iter()
                            .fold(0_i64, |acc, n| acc * 10 + (n - 48) as i64)
                    }),
            )
        })
        .collect()
}

pub fn get_sorted_distances(boxes: Vec<JunctionBox>) -> Vec<((usize, usize), u64)> {
    let mut distances: Vec<((usize, usize), u64)> = Vec::with_capacity(boxes.len().pow(2));

    for box1_i in 0..boxes.len() {
        for box2_i in box1_i + 1..boxes.len() {
            let distance = boxes[box1_i].distnace_squared(&boxes[box2_i]);
            distances.push(((box1_i, box2_i), distance));
        }
    }

    distances.sort_unstable_by_key(|d| d.1);
    distances
}

#[derive(PartialEq, Eq)]
pub struct BoxDistance {
    distance: u64,
    box1_i: usize,
    box2_i: usize,
}

impl Ord for BoxDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for BoxDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn get_sorted_distances_binary_heap(boxes: Vec<JunctionBox>) -> BinaryHeap<Reverse<BoxDistance>> {
    let mut distances: BinaryHeap<Reverse<BoxDistance>> = BinaryHeap::with_capacity(boxes.len().pow(2));

    for box1_i in 0..boxes.len() {
        for box2_i in box1_i + 1..boxes.len() {
            let distance = boxes[box1_i].distnace_squared(&boxes[box2_i]);
            distances.push(Reverse(BoxDistance {
                distance,
                box1_i,
                box2_i,
            }));
        }
    }

    distances
}

#[derive(Debug)]
struct DisjointSet {
    parent: Vec<usize>,
    sizes: Vec<usize>,
}

impl DisjointSet {
    fn new(count: usize) -> Self {
        Self {
            parent: (0..count).collect(),
            sizes: vec![1; count],
        }
    }

    fn find(&self, index: usize) -> usize {
        let mut current_index = index;
        while self.parent[current_index] != current_index {
            current_index = self.parent[current_index];
        }
        current_index
    }

    fn union(&mut self, index1: usize, index2: usize) {
        // print!("{index1:2} : {index2:2}\t");
        let parent1 = self.find(index1);
        let parent2 = self.find(index2);
        if parent1 == parent2 {
            // println!("Already connected:\t{index1:2} and {index2:2}");
            return;
        }
        // println!("Connecting:\t\t{parent2:2} to {parent1:2}");

        self.sizes[parent1] += self.sizes[parent2];
        self.sizes[parent2] = 0;
        self.parent[parent2] = parent1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
