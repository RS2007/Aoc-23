use std::{borrow::BorrowMut, collections::BTreeMap, fs};

fn part1(input: &str) -> u64 {
    input
        .split(",")
        .map(|split_string| {
            let a = split_string.chars().fold(0, |accum, x| {
                if x == '\n' {
                    return accum;
                }
                let mut hash = accum + x as u64;
                hash *= 17;
                hash = hash % 256;
                hash
            });
            a
        })
        .sum::<u64>()
}

fn part2(input: &str) -> u64 {
    input
        .split(",")
        .map(|split_string| {
            let (key, value) = if split_string.contains("=") {
                let split = split_string.split("=").collect::<Vec<&str>>();
                let key = split[0];
                let value = Some(split[1].parse::<u64>().unwrap());
                (key, value)
            } else {
                let split = split_string.split("-").collect::<Vec<&str>>();
                let key = split[0];
                (key, None)
            };
            let hash = key.chars().fold(0, |accum, x| {
                if x == '\n' {
                    return accum;
                }
                let mut hash = accum + x as u64;
                hash *= 17;
                hash = hash % 256;
                hash
            });
            (key, hash, value)
        })
        .fold(
            vec![Vec::<(&str, u64)>::new(); 256],
            |mut accum, (key, hash, value)| {
                if !value.is_none() {
                    if let Some(x) = accum[hash as usize].iter_mut().find(|x| x.0 == key) {
                        x.1 = value.unwrap();
                    } else {
                        accum[hash as usize].push((key, value.unwrap()));
                    }
                    return accum;
                }
                accum[hash as usize].retain(|x| x.0 != key);
                accum
            },
        )
        .iter()
        .enumerate()
        .map(|(box_indx, val)| {
            let a = val
                .iter()
                .enumerate()
                .map(|(indx, val)| (box_indx + 1) * (indx + 1) * val.1 as usize);
            a.sum::<usize>()
        })
        .sum::<usize>() as u64
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Expected file");
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_basic_hash() {
        assert_eq!(part1("HASH"), 52);
    }

    #[test]
    fn test_basic_hash2() {
        assert_eq!(part1("ot=7"), 231);
    }

    #[test]
    fn test_dev_part1() {
        let input = fs::read_to_string("./input.dev.txt").expect("Expected file");
        assert_eq!(part1(&input), 1320);
    }

    #[test]
    fn test_dev_part2() {
        let input = fs::read_to_string("./input.dev.txt").expect("Expected file");
        assert_eq!(part2(&input), 145);
    }
}
