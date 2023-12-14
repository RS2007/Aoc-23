use std::{collections::BTreeSet, fs};

fn vertical_score(grid: &Vec<Vec<char>>) -> u64 {
    let mut i: i32 = 0;
    let mut is_valid = true;
    while i < grid[0].len() as i32 - 1 {
        let mut j: i32 = 0;
        'jloop: while (0 <= (i - j) && (i - j) < grid[0].len() as i32)
            && (0 <= (i + j + 1))
            && ((i + j + 1) < grid[0].len() as i32)
        {
            for k in 0..grid.len() {
                let left_val = grid[k][(i - j) as usize];
                let right_val = grid[k][(i + j + 1) as usize];
                if left_val != right_val {
                    is_valid = false;
                    break 'jloop;
                }
            }
            j += 1;
        }
        if is_valid {
            return (i as u64) + 1;
        } else {
            i += 1;
            is_valid = true;
        }
    }
    horizontal_score(grid)
}

fn horizontal_score(grid: &Vec<Vec<char>>) -> u64 {
    let mut i: i32 = 0;
    let mut is_valid = true;
    while i < grid.len() as i32 - 1 {
        let mut j: i32 = 0;
        'jloop: while (0 <= (i - j) && (i - j) < grid.len() as i32)
            && (0 <= (i + j + 1))
            && ((i + j + 1) < grid.len() as i32)
        {
            for k in 0..grid[0].len() {
                let left_val = grid[(i - j) as usize][k];
                let right_val = grid[(i + j + 1) as usize][k];
                if left_val != right_val {
                    is_valid = false;
                    break 'jloop;
                }
            }
            j += 1;
        }
        if is_valid {
            return ((i as u64) + 1) * 100;
        } else {
            i += 1;
            is_valid = true;
        }
    }
    unreachable!();
}

fn modified_vertical(grid: &Vec<Vec<char>>) -> u64 {
    let mut i: i32 = 0;
    while i < grid[0].len() as i32 - 1 {
        let mut corrections: BTreeSet<usize> = BTreeSet::new();
        let mut j: i32 = 0;
        while (0 <= (i - j) && (i - j) < grid[0].len() as i32)
            && (0 <= (i + j + 1))
            && ((i + j + 1) < grid[0].len() as i32)
        {
            for k in 0..grid.len() {
                let left_val = grid[k][(i - j) as usize];
                let right_val = grid[k][(i + j + 1) as usize];
                if left_val != right_val {
                    corrections.insert(k);
                }
            }
            j += 1;
        }

        if corrections.len() == 1 {
            return (i as u64) + 1;
        }
        i += 1;
    }
    modified_horizontal(grid)
}

fn modified_horizontal(grid: &Vec<Vec<char>>) -> u64 {
    let mut i: i32 = 0;
    while i < grid.len() as i32 - 1 {
        let mut corrections: BTreeSet<usize> = BTreeSet::new();
        let mut j: i32 = 0;
        while (0 <= (i - j) && (i - j) < grid.len() as i32)
            && (0 <= (i + j + 1))
            && ((i + j + 1) < grid.len() as i32)
        {
            for k in 0..grid[0].len() {
                let left_val = grid[(i - j) as usize][k];
                let right_val = grid[(i + j + 1) as usize][k];
                if left_val != right_val {
                    corrections.insert(k);
                }
            }
            j += 1;
        }
        if corrections.len() == 1 {
            return ((i as u64) + 1) * 100;
        }
        i += 1;
    }
    unreachable!();
}

fn part1(input: &str) -> u64 {
    let parsed = input.split("\n\n").collect::<Vec<&str>>();
    let mut i = 0;
    let mut accum = 0;
    while i <= parsed.len() - 1 {
        let grid = parsed[i]
            .lines()
            .map(|character| character.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        accum += vertical_score(&grid);
        i += 1;
    }
    accum
}

fn part2(input: &str) -> u64 {
    let parsed = input.split("\n\n").collect::<Vec<&str>>();
    let mut i = 0;
    let mut accum = 0;
    while i <= parsed.len() - 1 {
        let grid = parsed[i]
            .lines()
            .map(|character| character.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        accum += modified_vertical(&grid);
        i += 1;
    }
    accum
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Expected file");
    println!("part1: {:?}", part1(&input));
    println!("part2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_dev_part1() {
        let input = fs::read_to_string("./input.dev.txt").expect("Expected file");
        assert_eq!(part1(&input), 405);
    }

    #[test]
    fn test_dev_part2() {
        let input = fs::read_to_string("./input.dev.txt").expect("Expected file");
        assert_eq!(part2(&input), 400);
    }
}
