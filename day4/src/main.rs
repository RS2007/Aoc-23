use std::{collections::BTreeMap, fs};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::map_res,
    multi::separated_list0,
    IResult,
};

fn parse_positive_integer(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_cards(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, parsed_cards) = separated_list0(multispace1, parse_positive_integer)(input)?;
    return Ok((input, parsed_cards));
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, _) = tag::<&str, &str, nom::error::Error<&str>>("Card ")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = parse_positive_integer(input)?;
    let (input, _) = tag::<&str, &str, nom::error::Error<&str>>(":")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, winning_cards) = parse_cards(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag::<&str, &str, nom::error::Error<&str>>("|")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, my_cards) = parse_cards(input)?;
    return Ok((input, (winning_cards, my_cards)));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, (winning_cards, my_cards)) = parse_input(line).unwrap();
            let nums = my_cards
                .iter()
                .filter(|card| winning_cards.contains(card))
                .count() as u32;
            match nums {
                0 => 0,
                _ => (2 as u32).pow(nums - 1),
            }
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let mut lookup: BTreeMap<u32, u32> = BTreeMap::new();
    let tot_cards = input.lines().enumerate().count();
    for i in 1..=tot_cards {
        lookup.insert(i as u32, 1);
    }
    input.lines().enumerate().for_each(|(indx, line)| {
        let indx = indx as u32;
        let (_, (winning_cards, my_cards)) = parse_input(line).unwrap();
        let nums = my_cards
            .iter()
            .filter(|card| winning_cards.contains(card))
            .count() as u32;
        let current_card_num = **(&lookup.get(&(indx + 1)).unwrap());
        for num in indx + 2..indx + nums + 2 {
            if let Some(value) = lookup.get_mut(&num) {
                *value += (1) * (current_card_num);
            }
        }
    });
    lookup.values().fold(0, |acc, val| acc + *val)
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Cannot read file to string");
    let start_time = std::time::Instant::now();
    println!("part1: {:?}", part1(&input));
    let end_time = std::time::Instant::now();
    println!("part1 took {:?}", end_time - start_time);
    let start_time = std::time::Instant::now();
    println!("part2: {:?}", part2(&input));
    let end_time = std::time::Instant::now();
    println!("part2 took {:?}", end_time - start_time);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_dev_part1() {
        let input = fs::read_to_string("./input.dev.txt").expect("Cannot read file to string");
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_dev_part2() {
        let input = fs::read_to_string("./input.dev.txt").expect("Cannot read file to string");
        assert_eq!(part2(&input), 30);
    }
}
