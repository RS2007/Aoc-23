use std::{collections::BTreeMap, fs};

#[derive(Debug)]
struct Game {
    moves: String,
    source_dest_map: BTreeMap<String, (String, String)>,
}

fn parse_input(input: &str) -> Game {
    let split_string = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            Some(line.to_string())
        })
        .collect::<Vec<String>>();
    let moves = split_string.iter().nth(0).unwrap();
    let source_dest_string = &split_string[1..];
    let source_dest_map = source_dest_string
        .iter()
        .fold(BTreeMap::new(), |mut acc, string| {
            let split_string = string.split(" = ").collect::<Vec<&str>>();
            let src = split_string.iter().nth(0).unwrap().to_string();
            let dest = split_string.iter().nth(1).unwrap();
            let dest_split = dest.split(", ").collect::<Vec<&str>>();
            let left_dest = &dest_split.iter().nth(0).unwrap()[1..];
            let right_dest = dest_split.iter().nth(1).unwrap();
            let right_dest = &right_dest[..right_dest.len() - 1];
            acc.insert(src, (left_dest.to_string(), right_dest.to_string()));
            acc
        });
    Game {
        moves: moves.to_string(),
        source_dest_map,
    }
}

fn part1(input: &str) -> u64 {
    let game = parse_input(input);
    let moves = game.moves.chars().collect::<Vec<char>>();
    let mut curr_location = "AAA";
    let destination = "ZZZ";
    let mut counter = 0;
    while curr_location != destination {
        let indx = counter % (moves.len());
        let curr_move = moves[indx];
        match curr_move {
            'L' => {
                curr_location = &game.source_dest_map[curr_location].0;
            }
            'R' => {
                curr_location = &game.source_dest_map[curr_location].1;
            }
            _ => {
                assert!(false, "Unreachable");
            }
        }
        counter += 1;
    }
    counter as u64
}

// From: https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn part2(input: &str) -> u64 {
    let game = parse_input(&input);
    let moves = game.moves.chars().collect::<Vec<char>>();
    let sources = game
        .source_dest_map
        .iter()
        .filter_map(|(key, _)| {
            if key.ends_with("A") {
                return Some(key.as_str());
            }
            None
        })
        .collect::<Vec<&str>>();
    // bfs

    let source_dests = sources
        .iter()
        .map(|source| {
            let mut curr_location = *source;
            let mut counter = 0;
            while !curr_location.ends_with("Z") {
                let indx = counter % (moves.len());
                let curr_move = moves[indx];
                match curr_move {
                    'L' => {
                        curr_location = &game.source_dest_map[curr_location].0;
                    }
                    'R' => {
                        curr_location = &game.source_dest_map[curr_location].1;
                    }
                    _ => {
                        assert!(false, "Unreachable");
                    }
                }
                counter += 1;
            }
            counter as u64
        })
        .collect::<Vec<u64>>();
    lcm(source_dests.as_ref())
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File expected");
    println!("part1: {:?}", part1(&input));
    println!("part2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_dev_part1() {
        let input = fs::read_to_string("./input.dev.txt").expect("File expected");
        let input2 = fs::read_to_string("./input.dev2.txt").expect("File expected");
        assert_eq!(part1(&input), 2);
        assert_eq!(part1(&input2), 6);
    }

    #[test]
    fn test_dev_part2() {
        let input = fs::read_to_string("./input.dev3.txt").expect("File expected");
        assert_eq!(part2(&input), 6);
    }
}
