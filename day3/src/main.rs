use std::{fs, collections::BTreeMap};

fn in_bounds(i: i32,j: i32,rows: usize,cols: usize) -> bool {
    (i < rows.try_into().unwrap() && i >= 0) && (j < cols.try_into().unwrap() && j >= 0)
}

fn part1(input: &str) -> u32 {
    let grid = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let rows = grid.len();
    let cols = grid.iter().nth(0).unwrap().len();
    let mut special_numbers: Vec<u32> = vec![];
    let mut num_accum = 0;
    let mut is_special = false;
    let moves: Vec<(i32, i32)> = vec![(0, -1), (-1, 0), (1, 0), (0, 1), (1, 1), (1, -1), (-1, -1), (-1, 1)];
    for i in 0..rows {
        for j in 0..cols {
            let current_elem = grid[i][j];
            if current_elem.is_digit(10) {
                for (del_x,del_y) in moves.iter() {
                    if in_bounds(i as i32 + del_x,j as i32 + del_y,rows,cols) {
                        if grid[(i as i32 + *del_x) as usize][(j as i32 + *del_y) as usize] != '.' && !grid[(i as i32 + *del_x) as usize][(j as i32 + *del_y) as usize].is_digit(10) {
                            is_special = true;
                        }
                    }
                }
                num_accum = num_accum*10 + current_elem.to_digit(10).unwrap();
            } else {
                if is_special {
                    special_numbers.push(num_accum);
                }
                num_accum = 0;
                is_special = false;
            }
        }
    }
    special_numbers.iter().sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let grid = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut gear_star:BTreeMap<(i32,i32),i32> = BTreeMap::new();
    let rows = grid.len();
    let cols = grid.iter().nth(0).unwrap().len();
    let moves: Vec<(i32, i32)> = vec![(0, -1), (-1, 0), (1, 0), (0, 1), (1, 1), (1, -1), (-1, -1), (-1, 1)];
    let mut accum = 0;
    let mut num_accum = 0;
    let mut current_star_index: Option<(i32,i32)> = None;
    for i in 0..rows {
        for j in 0..cols {
            let current_elem = grid[i][j];
            if current_elem.is_digit(10) {
                for (del_x,del_y) in moves.iter() {
                    if in_bounds(i as i32 + del_x,j as i32 + del_y,rows,cols) {
                        if grid[(i as i32 + *del_x) as usize][(j as i32 + *del_y) as usize] == '*' {
                            current_star_index = Some((i as i32 + *del_x,j as i32 + *del_y));
                        }
                    }
                }
                num_accum = num_accum*10 + current_elem.to_digit(10).unwrap();
            } else {
                if current_star_index.is_some() {
                    if gear_star.contains_key(&current_star_index.unwrap()) {
                        accum += num_accum as i32 *gear_star[&current_star_index.unwrap()];
                    }
                    gear_star.insert(current_star_index.unwrap(), num_accum.try_into().unwrap());
                }
                current_star_index = None;
                num_accum = 0;
            }
        }
    }
    accum as u32
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Cannot read file");
    println!("part1: {:?}",part1(&input));
    println!("part2: {:?}",part2(&input));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_dev_part1() {
        let input = fs::read_to_string("./input.dev.txt").expect("Cannot read file");
        assert_eq!(part1(&input),4361);
        
    }


    #[test]
    fn test_dev_part2() {
        let input = fs::read_to_string("./input.dev.txt").expect("Cannot read file");
        assert_eq!(part2(&input),467835);
        
    }
}
