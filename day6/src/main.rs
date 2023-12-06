use std::fs;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split("  ")
                .filter(|val| !val.is_empty())
                .map(|val| val.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}
fn parse_input2(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split("  ")
                .map(|val| val.trim())
                .filter(|val| !val.is_empty())
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<u64>>()
}

fn part1(input: &str) -> u32 {
    let parsed = parse_input(input);
    let times = &parsed[0];
    let distances = &parsed[1];
    assert_eq!(times.len(), distances.len());
    times
        .iter()
        .zip(distances.iter())
        .map(|x| {
            let current_time = *x.0;
            let current_distance = *x.1;
            let distances = (0..=current_time).map(|button_time| {
                let move_time = current_time - button_time;
                move_time * button_time
            });
            distances
                .filter(|distance| *distance > current_distance)
                .count()
        })
        .product::<usize>() as u32
}

fn part2(input: &str) -> u32 {
    let parsed = parse_input2(&input);
    let time = parsed[0];
    let curr_distance = parsed[1];

    let distances = (0..=time).map(|button_time| {
        let move_time = time - button_time;
        move_time * button_time
    });
    distances
        .filter(|distance| *distance > curr_distance)
        .count() as u32
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Cannot read file");
    println!("part1: {:?}", part1(&input));
    println!("part2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_dev_part1() {
        let input = fs::read_to_string("./input.dev.txt").expect("Cannot read file");
        assert_eq!(part1(&input), 288);
    }

    #[test]
    fn test_dev_part2() {
        let input = fs::read_to_string("./input.dev.txt").expect("Cannot read file");
        assert_eq!(part2(&input), 71503);
    }
}
