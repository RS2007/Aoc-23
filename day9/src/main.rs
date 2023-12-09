use std::fs;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|num| {
                    return num.parse::<i64>().unwrap();
                })
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> i64 {
    let parsed = parse_input(input);
    parsed
        .iter()
        .map(|arr| {
            let mut differences: Vec<Vec<i64>> = vec![arr.to_vec()];
            while !differences
                .iter()
                .last()
                .unwrap()
                .iter()
                .all(|val| *val == 0)
            {
                let difference_vec = differences
                    .iter()
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|x| x[1] - x[0])
                    .collect::<Vec<i64>>();
                differences.push(difference_vec);
            }
            differences
                .iter()
                .rev()
                .map(|val| val.last().unwrap())
                .sum::<i64>()
        })
        .sum::<i64>()
}

fn part2(input: &str) -> i64 {
    let parsed = parse_input(input);
    parsed
        .iter()
        .map(|arr| {
            let mut differences: Vec<Vec<i64>> = vec![arr.to_vec()];
            while !differences
                .iter()
                .last()
                .unwrap()
                .iter()
                .all(|val| *val == 0)
            {
                let difference_vec = differences
                    .iter()
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|x| x[1] - x[0])
                    .collect::<Vec<i64>>();
                differences.push(difference_vec);
            }
            differences.iter().rev().fold(0, |mut acc, x| {
                acc = x[0] - acc;
                acc
            })
        })
        .sum::<i64>()
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Expected file");
    println!("part1: {:?}", part1(&input));
    println!("part2: {:?}", part2(&input));
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_dev_part1() {
        let input = fs::read_to_string("./input.dev.txt").expect("Expected file");
        assert_eq!(part1(&input), 114);
    }

    #[test]
    fn test_dev_part2() {
        let input = fs::read_to_string("./input.dev.txt").expect("Expected file");
        assert_eq!(part2(&input), 2);
    }
}
