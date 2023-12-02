use std::{cmp::max, fs};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1, multispace0, multispace1, space0},
    combinator::{map_res, opt, verify},
    error::Error,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

fn parse_positive_integer(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

#[derive(Debug)]
struct Draw {
    blue: u32,
    red: u32,
    green: u32,
}

impl Draw {
    fn new() -> Self {
        return Draw {
            blue: 0,
            red: 0,
            green: 0,
        };
    }
}

fn parse_color(input: &str) -> IResult<&str, &str> {
    alt((tag("blue"), tag("red"), tag("green")))(input)
}
fn eat_whitespace(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

fn parse_num_color(input: &str) -> IResult<&str, (u32, &str)> {
    let (input, num) = parse_positive_integer(input)?;
    let (input, _) = eat_whitespace(input)?;
    let (input, color) = parse_color(input)?;
    return Ok((input, (num, color)));
}

fn parse_draw(input: &str) -> IResult<&str, Draw> {
    let (input, num_color_vec) = separated_list0(tag(", "), parse_num_color)(input)?;
    let mut draw = Draw::new();
    for (num, color) in num_color_vec.iter() {
        match *color {
            "blue" => {
                draw.blue = *num;
            }
            "red" => {
                draw.red = *num;
            }
            "green" => {
                draw.green = *num;
            }
            _ => {
                assert!(false, "Should not hit here");
            }
        }
    }
    return Ok((input, draw));
}

fn parse_all_draws(input: &str) -> IResult<&str, Draw> {
    let (input, all_draws) = separated_list0(tag("; "), parse_draw)(input)?;
    let mut whole_bag = Draw::new();
    for current_draw in all_draws.iter() {
        whole_bag.red = max(whole_bag.red, current_draw.red);
        whole_bag.blue = max(whole_bag.blue, current_draw.blue);
        whole_bag.green = max(whole_bag.green, current_draw.green);
    }

    Ok((input, whole_bag))
}

fn parse_input(input: &str) -> Vec<(u32, Draw)> {
    input
        .lines()
        .map(|line| {
            let line = tag::<&str, &str, nom::error::Error<&str>>("Game ")(line)
                .unwrap()
                .0;
            let (line, id) = parse_positive_integer(line).unwrap();
            let (line, _) = tag::<&str, &str, nom::error::Error<&str>>(": ")(line).unwrap();
            let (_line, bag) = parse_all_draws(line).unwrap();
            (id, bag)
        })
        .collect::<Vec<(u32, Draw)>>()
}

fn part1(input: &str) -> u32 {
    let bags = parse_input(input);
    bags.iter()
        .filter_map(|(id, bag)| {
            if (bag.red <= 12 && bag.green <= 13) && bag.blue <= 14 {
                return Some(*id);
            }
            None
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let bags = parse_input(input);
    bags.iter()
        .fold(0, |acc, (_, bag)| acc + bag.red * bag.blue * bag.green)
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("part 1: {:?}", part1(&input));
    println!("part 2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_dev_part1() {
        let input = fs::read_to_string("./input.dev.txt").expect("Unable to read file");
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn test_dev_part2() {
        let input = fs::read_to_string("./input.dev.txt").expect("Unable to read file");
        assert_eq!(part2(&input), 2286);
    }
}
