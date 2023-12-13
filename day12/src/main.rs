use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs,
};

trait Mergable {
    fn merge(&self) -> Vec<Vec<u64>>;
}

impl Mergable for Vec<u64> {
    fn merge(&self) -> Vec<Vec<u64>> {
        let mut return_vec: Vec<Vec<u64>> = vec![];
        let mut accumulator: Vec<u64> = vec![self[0]];
        for i in 1..self.len() {
            if self[i - 1] + 1 == self[i] {
                accumulator.push(self[i]);
            } else {
                return_vec.push(accumulator);
                accumulator = vec![self[i]];
            }
        }
        if !accumulator.is_empty() {
            return_vec.push(accumulator);
        }
        return_vec
    }
}
fn parse_input1(input: &str) -> Vec<(String, VecDeque<u64>)> {
    input
        .lines()
        .map(|line| {
            let split_string = line.split(" ").collect::<Vec<&str>>();
            let conditions = split_string
                .iter()
                .nth(1)
                .unwrap()
                .split(",")
                .map(|elem| elem.trim().parse::<u64>().unwrap())
                .collect::<VecDeque<u64>>();
            let spring = split_string[0];
            (spring.to_owned(), conditions)
        })
        .collect::<Vec<(String, VecDeque<u64>)>>()
}

fn parse_input2(input: &str) -> Vec<(String, VecDeque<u64>)> {
    input
        .lines()
        .map(|line| {
            let split_string = line.split(" ").collect::<Vec<&str>>();
            let conditions = split_string
                .iter()
                .nth(1)
                .unwrap()
                .split(",")
                .map(|elem| elem.trim().parse::<u64>().unwrap())
                .collect::<VecDeque<u64>>();
            let spring = split_string[0];
            let mut accum_string: String = "".to_string();
            let mut accum_vec: VecDeque<u64> = VecDeque::new();
            for i in 0..5 {
                accum_string += spring;
                if i != 4 {
                    accum_string += "?";
                }
            }
            for _ in 0..5 {
                for &condition in conditions.iter() {
                    accum_vec.push_back(condition);
                }
            }

            (accum_string.to_owned(), accum_vec)
        })
        .collect::<Vec<(String, VecDeque<u64>)>>()
}

fn backtrack_helper(
    current_state: &mut Vec<char>,
    q_indexes: &mut Vec<usize>,
    seen_indexes: &mut BTreeSet<usize>,
    seen: &mut BTreeSet<String>,
) {
    if seen_indexes.len() == q_indexes.len() {
        return;
    }
    for q_index in q_indexes.clone() {
        if !seen_indexes.contains(&q_index) {
            current_state[q_index] = '#';
            seen_indexes.insert(q_index);
            let collected_string = current_state.iter().collect::<String>();
            seen.insert(collected_string);
            backtrack_helper(current_state, q_indexes, seen_indexes, seen);
            seen_indexes.remove(&q_index);
            current_state[q_index] = '?';
        }
    }
}

fn part1_backtrack(input: &str) -> u64 {
    let parsed = parse_input1(input);
    parsed.iter().fold(0, |acc, (current_state, condition)| {
        let mut q_indexes = current_state
            .chars()
            .enumerate()
            .filter_map(|(indx, character)| {
                if character == '?' {
                    return Some(indx);
                }
                None
            })
            .collect::<Vec<usize>>();
        let mut seen_indices: BTreeSet<usize> = BTreeSet::new();
        let mut seen: BTreeSet<String> = BTreeSet::new();
        backtrack_helper(
            &mut current_state.chars().collect::<Vec<char>>(),
            &mut q_indexes,
            &mut seen_indices,
            &mut seen,
        );
        let a = seen
            .iter()
            .map(|seen_string| {
                seen_string
                    .chars()
                    .map(|character| {
                        if character == '?' {
                            return '.';
                        }
                        character
                    })
                    .collect::<String>()
            })
            .filter(|string| {
                let merged = string
                    .chars()
                    .enumerate()
                    .filter_map(|(indx, character)| {
                        if character == '#' {
                            return Some(indx as u64);
                        }
                        None
                    })
                    .collect::<Vec<u64>>()
                    .merge();
                if merged.len() != condition.len() {
                    return false;
                }
                merged
                    .iter()
                    .map(|vector| vector.len())
                    .zip(condition.iter())
                    .all(|(a, &b)| a as u64 == b)
            });

        let b = a.count() as u64;
        acc + b
    })
}

fn recurse(
    start_indx: usize,
    condq: &mut VecDeque<u64>,
    spring: &str,
    memo: &mut BTreeMap<(usize, String), u64>,
) -> u64 {
    let joined_condq = condq
        .iter()
        .map(|integer| integer.to_string())
        .collect::<String>();
    if memo.contains_key(&(start_indx, joined_condq.clone())) {
        return *memo.get(&(start_indx, joined_condq)).unwrap();
    }
    if condq.is_empty() && start_indx > spring.len() - 1 {
        return 1;
    }

    if condq.is_empty() {
        if spring[start_indx..].chars().all(|x| x != '#') {
            return 1;
        }
        return 0;
    }

    if start_indx > spring.len() - 1 {
        return 0;
    }

    match spring.chars().nth(start_indx).unwrap() {
        '.' => {
            let value = recurse(1 + start_indx, condq, spring, memo);
            memo.insert(
                (
                    start_indx,
                    condq
                        .iter()
                        .map(|integer| integer.to_string())
                        .collect::<String>(),
                ),
                value,
            );
            return value;
        }
        '#' => {
            let old_cond_q = condq.clone();
            let old_cond_q_str = old_cond_q
                .iter()
                .map(|integer| integer.to_string())
                .collect::<String>();
            let top_of_q = condq.pop_front().unwrap();
            // TODO: process this contigously instead of doing recursive calls
            for i in 0..top_of_q {
                if let Some(a) = spring.chars().nth(start_indx + i as usize) {
                    match a {
                        '#' | '?' => {
                            continue;
                        }
                        '.' => {
                            let value = 0;
                            memo.insert((start_indx, old_cond_q_str), value);
                            return value;
                        }
                        _ => {
                            unreachable!();
                        }
                    }
                } else {
                    let value = 0;
                    memo.insert((start_indx, old_cond_q_str), value);
                    return value;
                }
            }
            if let Some(b) = spring.chars().nth(start_indx + top_of_q as usize) {
                match b {
                    '#' => {
                        let value = 0;
                        memo.insert((start_indx, old_cond_q_str), value);
                        return value;
                    }
                    '?' => {
                        let value =
                            recurse(start_indx + top_of_q as usize + 1, condq, spring, memo);
                        memo.insert((start_indx, old_cond_q_str), value);
                        return value;
                    }
                    '.' => {}
                    _ => {
                        unreachable!();
                    }
                }
            }
            let value = recurse(top_of_q as usize + start_indx, condq, spring, memo);
            memo.insert((start_indx, old_cond_q_str), value);
            return value;
        }
        '?' => {
            let mut old_condq = condq.clone();
            let old_condq_str = old_condq
                .iter()
                .map(|integer| integer.to_string())
                .collect::<String>();
            let top_of_q = condq.pop_front().unwrap();
            // TODO: process this contigously instead of doing recursive calls
            let mut has_seen_h = false;
            for i in 0..top_of_q {
                if let Some(a) = spring.chars().nth(start_indx + i as usize) {
                    match a {
                        '#' => {
                            has_seen_h = true;
                            continue;
                        }
                        '?' => {
                            continue;
                        }
                        '.' => {
                            if !has_seen_h {
                                let value =
                                    recurse(start_indx + i as usize, &mut old_condq, spring, memo);
                                memo.insert((start_indx, old_condq_str), value);
                                return value;
                            }
                            let value = 0;
                            memo.insert((start_indx, old_condq_str), value);
                            return value;
                        }
                        _ => {
                            unreachable!();
                        }
                    }
                } else {
                    return 0;
                }
            }

            if let Some(b) = spring.chars().nth(start_indx + top_of_q as usize) {
                match b {
                    '#' => {
                        let value = recurse(start_indx + 1, &mut old_condq, spring, memo);
                        memo.insert((start_indx, old_condq_str), value);
                        return value;
                    }
                    '?' => {}
                    _ => {}
                }
            }
            let value = recurse(1 + top_of_q as usize + start_indx, condq, spring, memo)
                + recurse(start_indx + 1, &mut old_condq, spring, memo);
            memo.insert((start_indx, old_condq_str), value);
            return value;
        }
        _ => {
            unreachable!();
        }
    };
}

fn part1_memo(input: &str) -> u64 {
    let parsed = parse_input1(input);
    let nums = parsed
        .iter()
        .map(|(current_state, condition)| {
            let mut memo: BTreeMap<(usize, String), u64> = BTreeMap::new();
            let a = recurse(0, &mut condition.clone(), current_state, &mut memo);
            a
        })
        .collect::<Vec<u64>>();
    nums.iter().sum::<u64>()
}

fn part2(input: &str) -> u64 {
    let parsed = parse_input2(input);
    let nums = parsed
        .iter()
        .map(|(current_state, condition)| {
            let mut memo: BTreeMap<(usize, String), u64> = BTreeMap::new();
            let a = recurse(0, &mut condition.clone(), current_state, &mut memo);
            a
        })
        .collect::<Vec<u64>>();
    nums.iter().sum::<u64>()
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Expected file");
    println!("part1: {:?}", part1_memo(&input));
    println!("part2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn test_dev_part1_backtrack() {
        let input = fs::read_to_string("./input.dev.txt").expect("Expected file");
        assert_eq!(part1_backtrack(&input), 21);
    }

    #[test]
    fn test_dev_part1() {
        let input = fs::read_to_string("./input.dev.txt").expect("Expected file");
        // assert_eq!(
        //     recurse(
        //         13,
        //         &mut vec![1 as u64].into_iter().collect::<VecDeque<u64>>(),
        //         ".??..??...?##."
        //     ),
        //     1
        // );

        let input2 = "????#.##??###???#?#? 2,3,4,4";
        let input3 = "????.######..#####. 1,6,5";
        assert_eq!(part1_memo(&input), 21);
        assert_eq!(part1_memo(&input2), 2);
        assert_eq!(part1_memo(&input3), 4);
    }

    #[test]
    fn test_parse_2() {
        let input = "???.### 1,1,3";
        assert_eq!(
            parse_input2(&input)[0].0,
            "???.###????.###????.###????.###????.###"
        );
    }

    #[test]
    fn test_dev_part2() {
        let input = fs::read_to_string("./input.dev.txt").expect("Expected file");
        assert_eq!(part2(&input), 525152);
    }
}
