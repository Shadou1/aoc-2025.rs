use std::{
    collections::{HashMap, VecDeque},
    fmt, iter,
};

pub mod part1;
pub mod part2;

pub fn parse_devices_hashmap(input: &str) -> HashMap<[u8; 3], Vec<[u8; 3]>> {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let id = bytes[..3].try_into().unwrap();
            let outputs = bytes[5..]
                .split(|&byte| byte == b' ')
                .map(|output| output.try_into().unwrap())
                .collect();
            (id, outputs)
        })
        .collect()
}

pub const fn id_to_index(id: &[u8]) -> usize {
    (id[0] as usize - 97) * 26usize.pow(2) + (id[1] as usize - 97) * 26 + (id[2] as usize - 97)
}

pub fn index_to_id(index: usize) -> String {
    format!(
        "{}{}{}",
        (index / (26 * 26) % 26 + 97) as u8 as char,
        (index / 26 % 26 + 97) as u8 as char,
        (index % 26 + 97) as u8 as char,
    )
}

pub fn parse_devices_vec(input: &str) -> Vec<Vec<usize>> {
    let mut id_to_outputs = Vec::new();
    id_to_outputs.resize(26 * 26 * 26, Vec::with_capacity(4));
    for (id, outputs) in input.lines().map(|line| {
        let bytes = line.as_bytes();
        let id = id_to_index(bytes[..3].try_into().unwrap());
        let output_ids = bytes[5..].split(|&byte| byte == b' ').map(id_to_index);
        (id, output_ids)
    }) {
        id_to_outputs[id].extend(outputs);
    }
    id_to_outputs
}

#[derive(Clone)]
struct Path {
    count: usize,
    main: Vec<usize>,
    extra: Vec<Path>,
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let main: Vec<String> = self.main.iter().map(|&index| index_to_id(index)).collect();
        if self.extra.is_empty() {
            write!(f, "{{{} M: {:?}}}", self.count, main)
        } else {
            write!(f, "{{{} M: {:?},\n\tE:{:?}}}", self.count, main, self.extra)
        }
    }
}

impl Path {
    fn containing_path(&mut self, node: usize) -> Option<&mut Path> {
        if self.main.contains(&node) {
            Some(self)
        } else {
            let mut path = None;
            for extra in &mut self.extra {
                path = extra.containing_path(node);
                if path.is_some() {
                    break;
                }
            }
            path
        }
    }

    fn containing_path_all(&mut self, node: usize) -> Vec<&mut Path> {
        let mut paths = Vec::new();
        if self.main.contains(&node) {
            paths.push(self);
        } else {
            for extra in &mut self.extra {
                paths.append(&mut extra.containing_path_all(node));
            }
        }
        paths
    }

    fn add_extra(&mut self, extra: Path) -> usize {
        let mut to_add = 1;
        for extra_i in 1..extra.main.len() - 1 {
            let main_i = self
                .main
                .iter()
                .position(|&index| index == extra.main[extra_i]);
            if let Some(main_i) = main_i
                && main_i < self.main.len() - 1
            {
                if self.main[main_i] == extra.main[extra_i]
                    && self.main[main_i + 1] != extra.main[extra_i + 1]
                {
                    to_add *= 2;
                }
            }
        }
        self.count += to_add;
        self.extra.push(extra);
        return to_add;
    }

    fn pretty_print(&self, mut tab: usize) {
        tab += 1;
        let main: Vec<String> = self.main.iter().map(|&index| index_to_id(index)).collect();
        println!("{} M: {:?}", self.count, main);
        for extra in &self.extra {
            print!("{}E: ", "\t".repeat(tab));
            extra.pretty_print(tab);
        }
    }

    fn count(&self, mut sum: usize) -> usize {
        sum += 1;
        if !self.extra.is_empty() {
            for extra in &self.extra {
                sum += extra.count(0);
            }
        }
        sum
    }

    // fn count_muls(&self, muls: &mut [u64], count: u64, parent: Option<&Path>) -> u64 {
    //     if let Some(parent) = parent {
    //         let mut current_muls = vec![0; 26_usize.pow(3)];
    //         let mut current_count = 1;
    //
    //         if self.extra.is_empty() {
    //             for i in 0..self.main.len() - 1 {
    //                 if self.main[i] == parent.main[i] {
    //                     current_muls[i] = 1;
    //                     if self.main[i + 1] == parent.main[i + 1] {
    //                         // current_muls[i] = 1;
    //                     } else if self.main[i + 1] != parent.main[i + 1] {
    //                         current_count += 1;
    //                         current_muls[i] *= 2;
    //                     } else {
    //                         current_muls[i] += 1;
    //                     }
    //                 }
    //             }
    //         } else {
    //             for extra in &self.extra {
    //                 current_count += extra.count_muls(muls, 0, Some(&self))
    //             }
    //         }
    //
    //         current_muls.iter().enumerate().for_each(|(index, mul)| {
    //             muls[index] += mul;
    //         });
    //         count + current_count
    //     } else {
    //         let mut count = 1;
    //         self.main
    //             .iter()
    //             .take(self.main.len() - 1)
    //             .for_each(|&index| muls[index] += 1);
    //         for extra in &self.extra {
    //             count += extra.count_muls(muls, count, Some(&self))
    //         }
    //         count
    //     }
    // }
}

fn path_count(outputs: &[Vec<usize>], counts: &mut Vec<u64>, node: usize) -> u64 {
    if counts[node] != u64::MAX {
        return counts[node];
    }

    counts[node] = outputs[node]
        .iter()
        .map(|&output| path_count(outputs, counts, output))
        .sum();

    counts[node]
}

pub fn dfs(outputs: &[Vec<usize>], counts: &mut Vec<u64>, start: usize, end: usize) -> u64 {
    counts[start] = u64::MAX;
    counts.iter_mut().for_each(|count| {
        if *count == 0 {
            *count = u64::MAX
        }
    });
    counts[end] = 1;
    path_count(outputs, counts, start)
}

pub fn bfs(outputs: &[Vec<usize>], start: usize, end: usize) -> u64 {
    let start = Path {
        count: 1,
        main: vec![start],
        extra: vec![],
    };
    let mut to_visit = VecDeque::new();
    to_visit.push_back(start);
    let mut shortest_path: Option<Path> = None;
    let mut visited: Vec<bool> = vec![false; 26_usize.pow(3)];

    while let Some(path) = to_visit.pop_front() {
        println!();
        println!("Current:  {:#?}", path);

        let &last_node = path.main.last().expect("Path is empty");
        if visited[last_node] {
            println!("Already visited");

            // TODO refactor
            // Find first path containing curren path
            // if let Some(shortest) = &mut shortest_path {
            //     let mut to_search = iter::once(shortest).chain(to_visit.iter_mut());
            //     let Some(parent_path) = to_search.find_map(|path| path.containing_path(last_node))
            //     else {
            //         panic!("Visited node is not part of any path");
            //     };
            //     parent_path.add_extra(path);
            // } else {
            //     let mut to_search = to_visit.iter_mut();
            //     let Some(parent_path) = to_search.find_map(|path| path.containing_path(last_node))
            //     else {
            //         panic!("Visited node is not part of any path");
            //     };
            //     parent_path.add_extra(path);
            // }

            // Find all paths containing current path
            if let Some(shortest) = &mut shortest_path {
                let to_search = iter::once(shortest).chain(to_visit.iter_mut());
                for search_path in to_search {
                    let mut to_add = 0;
                    for parent_path in search_path.containing_path_all(last_node) {
                        to_add += parent_path.add_extra(path.clone());
                    }
                    // search_path.count += to_add;
                }
            } else {
                let to_search = to_visit.iter_mut();
                for search_path in to_search {
                    let mut to_add = 0;
                    for parent_path in search_path.containing_path_all(last_node) {
                        to_add += parent_path.add_extra(path.clone());
                    }
                    // search_path.count += to_add;
                }
            }

            println!("To visit: {:#?}", to_visit);
            if let Some(shortest) = &shortest_path {
                println!("Shortest: {:#?}", shortest);
            }

            continue;
        }
        visited[last_node] = true;

        if last_node == end {
            println!("Found shortest path to end");
            shortest_path = Some(path);
            continue;
        }

        for &output in &outputs[last_node] {
            let mut new_nodes = path.main.clone();
            new_nodes.push(output);
            let new_path = Path {
                count: path.count,
                main: new_nodes,
                extra: path.extra.clone(),
            };
            to_visit.push_back(new_path);
        }

        println!("To visit: {:#?}", to_visit);
        if let Some(shortest) = &shortest_path {
            println!("Shortest: {:#?}", shortest);
        }
    }

    let Some(shortest_path) = shortest_path else {
        panic!("Couldn't find path from start to end");
    };

    println!();
    shortest_path.pretty_print(0);
    println!("{}", shortest_path.count);

    // print!("[ ");
    // for (index, &mul) in muls.iter().enumerate() {
    //     if mul > 0 {
    //         print!("{}: {}, ", index_to_id(index), mul);
    //     }
    // }
    // println!("]")

    shortest_path.count as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_to_index() {
        let mut index_actual = 0;
        for i1 in b'a'..=b'z' {
            for i2 in b'a'..=b'z' {
                for i3 in b'a'..=b'z' {
                    let id = &[i1, i2, i3];
                    let index_generated = id_to_index(id);
                    assert_eq!(index_generated, index_actual);
                    index_actual += 1;
                }
            }
        }
    }

    #[test]
    fn test_bfs_part1() {
        let input = include_str!("../input-test-part1.txt");
        let expected = 5;
        let outputs = parse_devices_vec(input);
        let result = bfs(&outputs, id_to_index(b"you"), id_to_index(b"out"));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bfs_part2() {
        let input = include_str!("../input-test-part2.txt");
        let expected = 8;
        let outputs = parse_devices_vec(input);
        let result = bfs(&outputs, id_to_index(b"svr"), id_to_index(b"out"));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bfs_test_input() {
        let input = include_str!("../input-test.txt");
        let expected = 5;
        let outputs = parse_devices_vec(input);
        let result = bfs(&outputs, id_to_index(b"you"), id_to_index(b"out"));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dfs_part1() {
        let input = include_str!("../input-test-part1.txt");
        let expected = 5;
        let outputs = parse_devices_vec(input);
        let mut counts = vec![u64::MAX; 26_usize.pow(3)];
        let result = dfs(
            &outputs,
            &mut counts,
            id_to_index(b"you"),
            id_to_index(b"out"),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dfs_part2() {
        let input = include_str!("../input-test-part2.txt");
        let expected = 8;
        let outputs = parse_devices_vec(input);
        let mut counts = vec![u64::MAX; 26_usize.pow(3)];
        let result = dfs(
            &outputs,
            &mut counts,
            id_to_index(b"svr"),
            id_to_index(b"out"),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dfs_test_input() {
        let input = include_str!("../input-test.txt");
        let expected = 5;
        let outputs = parse_devices_vec(input);
        let mut counts = vec![u64::MAX; 26_usize.pow(3)];
        let result = dfs(
            &outputs,
            &mut counts,
            id_to_index(b"you"),
            id_to_index(b"out"),
        );
        assert_eq!(result, expected);
    }
}
