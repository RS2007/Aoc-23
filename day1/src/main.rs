use std::fs;

fn part1(input: &str) -> i32 {
    input
        .lines()
        .into_iter()
        .map(|x| {
            let first_num = x.chars().filter_map(|y| y.to_digit(10)).nth(0).unwrap();
            let last_num = x
                .chars()
                .rev()
                .filter_map(|y| y.to_digit(10))
                .nth(0)
                .unwrap();

            return (first_num, last_num);
        })
        .fold(0, |acc, (first, last)| {
            acc + ((first as i32) * 10 + last as i32)
        })
}

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn part2(input: &str) -> i32 {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let mut min_index_till_now = usize::MAX;
            let mut min_num_till_now = 0 as usize;
            let mut min_index_till_now_rev = usize::MAX;
            let mut min_num_till_now_rev = 0 as usize;
            let f_num_opt = line
                .chars()
                .enumerate()
                .filter(|(_, y)| y.is_ascii_digit())
                .nth(0);
            let l_num_opt = line
                .chars()
                .rev()
                .enumerate()
                .filter(|(_, y)| y.is_ascii_digit())
                .nth(0);
            NUMS.iter().enumerate().for_each(|(idx, &num)| {
                let matched_index_str_tuple = line.match_indices(num).collect::<Vec<_>>();
                if let Some((matched_index, _)) = matched_index_str_tuple.iter().nth(0) {
                    if *matched_index < min_index_till_now {
                        min_index_till_now = *matched_index;
                        min_num_till_now = idx + 1;
                    }
                }
                let reversed_line = line.chars().rev().collect::<String>();
                let reversed_num = num.chars().rev().collect::<String>();
                let matched_index_str_tuple_rev = reversed_line
                    .match_indices(reversed_num.as_str())
                    .collect::<Vec<_>>();
                if let Some((matched_index, _)) = matched_index_str_tuple_rev.iter().nth(0) {
                    if *matched_index < min_index_till_now_rev {
                        min_index_till_now_rev = *matched_index;
                        min_num_till_now_rev = idx + 1;
                    }
                }
            });
            if f_num_opt.is_some() {
                let (first_num_idx, first_num) = f_num_opt.unwrap();
                if first_num_idx < min_index_till_now {
                    min_num_till_now = first_num.to_digit(10).unwrap() as usize;
                }
            }
            if l_num_opt.is_some() {
                let (last_num_idx, last_num) = l_num_opt.unwrap();
                if last_num_idx < min_index_till_now_rev {
                    min_num_till_now_rev = last_num.to_digit(10).unwrap() as usize;
                }
            }
            return (min_num_till_now, min_num_till_now_rev);
        })
        .fold(0, |acc, (first, last)| acc + first * 10 + last) as i32
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Cannot read file to string");
    println!("part 1: {:?}", part1(&input));
    println!("part 2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_dev_part1() {
        let input = fs::read_to_string("./input.dev.txt").expect("Cannot read file to string");
        assert_eq!(part1(&input), 142);
    }
    #[test]
    fn test_dev_part2() {
        let input =
            fs::read_to_string("./input_part2.dev.txt").expect("Cannot read file to string");
        assert_eq!(part2(&input), 281);
    }
}
