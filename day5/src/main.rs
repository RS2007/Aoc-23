use std::fs;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0, newline};
use nom::combinator::map_res;
use nom::{multi::separated_list0, sequence::preceded, IResult};

#[derive(Debug)]
struct FieldMap {
    seeds: Vec<u64>,
    seed_soil_vec: Vec<Vec<u64>>,
    soil_fert_vec: Vec<Vec<u64>>,
    fert_water_vec: Vec<Vec<u64>>,
    water_light_vec: Vec<Vec<u64>>,
    light_temp_vec: Vec<Vec<u64>>,
    temp_humidity_vec: Vec<Vec<u64>>,
    humidity_loc_vec: Vec<Vec<u64>>,
}

fn parse_positive_integer(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(input)
}

fn parse_input(input: &str) -> IResult<&str, FieldMap> {
    let (input, seeds) = preceded(
        tag("seeds: "),
        separated_list0(multispace0, parse_positive_integer),
    )(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("\nseed-to-soil map:\n")(input)?;
    let (input, seed_soil_vec) =
        separated_list0(tag("\n"), separated_list0(tag(" "), parse_positive_integer))(input)?;
    let seed_soil_vec = &seed_soil_vec
        .iter()
        .filter_map(|vect| {
            if !vect.is_empty() {
                return Some(vect.to_vec());
            }
            None
        })
        .collect::<Vec<Vec<u64>>>();
    let (input, _) = tag("soil-to-fertilizer map:\n")(input)?;
    let (input, soil_fert_vec) =
        separated_list0(tag("\n"), separated_list0(tag(" "), parse_positive_integer))(input)?;
    let soil_fert_vec = &soil_fert_vec
        .iter()
        .filter_map(|vect| {
            if !vect.is_empty() {
                return Some(vect.to_vec());
            }
            None
        })
        .collect::<Vec<Vec<u64>>>();
    let (input, _) = tag("fertilizer-to-water map:\n")(input)?;
    let (input, fert_water_vec) =
        separated_list0(tag("\n"), separated_list0(tag(" "), parse_positive_integer))(input)?;
    let fert_water_vec = &fert_water_vec
        .iter()
        .filter_map(|vect| {
            if !vect.is_empty() {
                return Some(vect.to_vec());
            }
            None
        })
        .collect::<Vec<Vec<u64>>>();
    let (input, _) = tag("water-to-light map:\n")(input)?;
    let (input, water_light_vec) =
        separated_list0(tag("\n"), separated_list0(tag(" "), parse_positive_integer))(input)?;
    let water_light_vec = &water_light_vec
        .iter()
        .filter_map(|vect| {
            if !vect.is_empty() {
                return Some(vect.to_vec());
            }
            None
        })
        .collect::<Vec<Vec<u64>>>();
    let (input, _) = tag("light-to-temperature map:\n")(input)?;
    let (input, light_temp_vec) =
        separated_list0(tag("\n"), separated_list0(tag(" "), parse_positive_integer))(input)?;
    let light_temp_vec = &light_temp_vec
        .iter()
        .filter_map(|vect| {
            if !vect.is_empty() {
                return Some(vect.to_vec());
            }
            None
        })
        .collect::<Vec<Vec<u64>>>();
    let (input, _) = tag("temperature-to-humidity map:\n")(input)?;
    let (input, temp_humidity_vec) =
        separated_list0(tag("\n"), separated_list0(tag(" "), parse_positive_integer))(input)?;
    let temp_humidity_vec = &temp_humidity_vec
        .iter()
        .filter_map(|vect| {
            if !vect.is_empty() {
                return Some(vect.to_vec());
            }
            None
        })
        .collect::<Vec<Vec<u64>>>();
    let (input, _) = tag("humidity-to-location map:\n")(input)?;
    let (input, humidity_loc_vec) =
        separated_list0(tag("\n"), separated_list0(tag(" "), parse_positive_integer))(input)?;
    let humidity_loc_vec = &humidity_loc_vec
        .iter()
        .filter_map(|vect| {
            if !vect.is_empty() {
                return Some(vect.to_vec());
            }
            None
        })
        .collect::<Vec<Vec<u64>>>()
        .to_vec();
    return Ok((
        input,
        FieldMap {
            seeds,
            seed_soil_vec: seed_soil_vec.to_vec(),
            soil_fert_vec: soil_fert_vec.to_vec(),
            humidity_loc_vec: humidity_loc_vec.to_vec(),
            temp_humidity_vec: temp_humidity_vec.to_vec(),
            light_temp_vec: light_temp_vec.to_vec(),
            water_light_vec: water_light_vec.to_vec(),
            fert_water_vec: fert_water_vec.to_vec(),
        },
    ));
}

fn part1(input: &str) -> u64 {
    let (_, field_map) = parse_input(input).unwrap();
    field_map
        .seeds
        .iter()
        .map(|seed| {
            let seed_val = seed;
            let soil_val = field_map.seed_soil_vec.iter().fold(0, |accum, vect| {
                let source_start = *vect.iter().nth(1).unwrap();
                let dest_start = *vect.iter().nth(0).unwrap();
                let source_increment = *vect.iter().nth(2).unwrap();
                if (source_start..source_start + source_increment).contains(&seed_val) {
                    return accum + dest_start + seed_val - source_start;
                }
                accum
            });
            let soil_val = match soil_val {
                0 => *seed_val,
                _ => soil_val,
            };
            let fertilizer_val = field_map.soil_fert_vec.iter().fold(0, |accum, vect| {
                let source_start = *vect.iter().nth(1).unwrap();
                let dest_start = *vect.iter().nth(0).unwrap();
                let source_increment = *vect.iter().nth(2).unwrap();
                if (source_start..source_start + source_increment).contains(&soil_val) {
                    return accum + dest_start + soil_val - source_start;
                }
                accum
            });
            let fertilizer_val = match fertilizer_val {
                0 => soil_val,
                _ => fertilizer_val,
            };
            let water_val = field_map.fert_water_vec.iter().fold(0, |accum, vect| {
                let source_start = *vect.iter().nth(1).unwrap();
                let dest_start = *vect.iter().nth(0).unwrap();
                let source_increment = *vect.iter().nth(2).unwrap();
                if (source_start..source_start + source_increment).contains(&fertilizer_val) {
                    return accum + dest_start + fertilizer_val - source_start;
                }
                accum
            });
            let water_val = match water_val {
                0 => fertilizer_val,
                _ => water_val,
            };
            let light_val = field_map.water_light_vec.iter().fold(0, |accum, vect| {
                let source_start = *vect.iter().nth(1).unwrap();
                let dest_start = *vect.iter().nth(0).unwrap();
                let source_increment = *vect.iter().nth(2).unwrap();
                if (source_start..source_start + source_increment).contains(&water_val) {
                    return accum + dest_start + water_val - source_start;
                }
                accum
            });
            let light_val = match light_val {
                0 => water_val,
                _ => light_val,
            };

            let temp_val = field_map.light_temp_vec.iter().fold(0, |accum, vect| {
                let source_start = *vect.iter().nth(1).unwrap();
                let dest_start = *vect.iter().nth(0).unwrap();
                let source_increment = *vect.iter().nth(2).unwrap();
                if (source_start..source_start + source_increment).contains(&light_val) {
                    return accum + dest_start + light_val - source_start;
                }
                accum
            });
            let temp_val = match temp_val {
                0 => light_val,
                _ => temp_val,
            };

            let humidity_val = field_map.temp_humidity_vec.iter().fold(0, |accum, vect| {
                let source_start = *vect.iter().nth(1).unwrap();
                let dest_start = *vect.iter().nth(0).unwrap();
                let source_increment = *vect.iter().nth(2).unwrap();
                if (source_start..source_start + source_increment).contains(&temp_val) {
                    return accum + dest_start + temp_val - source_start;
                }
                accum
            });
            let humidity_val = match humidity_val {
                0 => temp_val,
                _ => humidity_val,
            };
            let loc_val = field_map.humidity_loc_vec.iter().fold(0, |accum, vect| {
                let source_start = *vect.iter().nth(1).unwrap();
                let dest_start = *vect.iter().nth(0).unwrap();
                let source_increment = *vect.iter().nth(2).unwrap();
                if (source_start..source_start + source_increment).contains(&humidity_val) {
                    return accum + dest_start + humidity_val - source_start;
                }
                accum
            });
            let loc_val = match loc_val {
                0 => humidity_val,
                _ => loc_val,
            };
            loc_val
        })
        .min()
        .unwrap()
        .into()
}

fn part2(input: &str) -> u64 {
    let (_, field_map) = parse_input(input).unwrap();
    let a = field_map
        .seeds
        .chunks(2)
        .flat_map(|x| {
            let start_seed = x[0];
            let seek_num = x[1];
            start_seed..start_seed + seek_num
        })
        .map(|seed| {
            let seed_val = seed;
            let soil_val = field_map.seed_soil_vec.iter().fold(0, |accum, vect| {
                let source_start = *vect.iter().nth(1).unwrap();
                let dest_start = *vect.iter().nth(0).unwrap();
                let source_increment = *vect.iter().nth(2).unwrap();
                if (source_start..source_start + source_increment).contains(&seed_val) {
                    return accum + dest_start + seed_val - source_start;
                }
                accum
            });
            let soil_val = match soil_val {
                0 => seed_val,
                _ => soil_val,
            };
            let fertilizer_val = field_map.soil_fert_vec.iter().fold(0, |accum, vect| {
                let source_start = *vect.iter().nth(1).unwrap();
                let dest_start = *vect.iter().nth(0).unwrap();
                let source_increment = *vect.iter().nth(2).unwrap();
                if (source_start..source_start + source_increment).contains(&soil_val) {
                    return accum + dest_start + soil_val - source_start;
                }
                accum
            });
            let fertilizer_val = match fertilizer_val {
                0 => soil_val,
                _ => fertilizer_val,
            };
            let water_val = field_map.fert_water_vec.iter().fold(0, |accum, vect| {
                let source_start = *vect.iter().nth(1).unwrap();
                let dest_start = *vect.iter().nth(0).unwrap();
                let source_increment = *vect.iter().nth(2).unwrap();
                if (source_start..source_start + source_increment).contains(&fertilizer_val) {
                    return accum + dest_start + fertilizer_val - source_start;
                }
                accum
            });
            let water_val = match water_val {
                0 => fertilizer_val,
                _ => water_val,
            };
            let light_val = field_map.water_light_vec.iter().fold(0, |accum, vect| {
                let source_start = *vect.iter().nth(1).unwrap();
                let dest_start = *vect.iter().nth(0).unwrap();
                let source_increment = *vect.iter().nth(2).unwrap();
                if (source_start..source_start + source_increment).contains(&water_val) {
                    return accum + dest_start + water_val - source_start;
                }
                accum
            });
            let light_val = match light_val {
                0 => water_val,
                _ => light_val,
            };

            let temp_val = field_map.light_temp_vec.iter().fold(0, |accum, vect| {
                let source_start = *vect.iter().nth(1).unwrap();
                let dest_start = *vect.iter().nth(0).unwrap();
                let source_increment = *vect.iter().nth(2).unwrap();
                if (source_start..source_start + source_increment).contains(&light_val) {
                    return accum + dest_start + light_val - source_start;
                }
                accum
            });
            let temp_val = match temp_val {
                0 => light_val,
                _ => temp_val,
            };

            let humidity_val = field_map.temp_humidity_vec.iter().fold(0, |accum, vect| {
                let source_start = *vect.iter().nth(1).unwrap();
                let dest_start = *vect.iter().nth(0).unwrap();
                let source_increment = *vect.iter().nth(2).unwrap();
                if (source_start..source_start + source_increment).contains(&temp_val) {
                    return accum + dest_start + temp_val - source_start;
                }
                accum
            });
            let humidity_val = match humidity_val {
                0 => temp_val,
                _ => humidity_val,
            };
            let loc_val = field_map.humidity_loc_vec.iter().fold(0, |accum, vect| {
                let source_start = *vect.iter().nth(1).unwrap();
                let dest_start = *vect.iter().nth(0).unwrap();
                let source_increment = *vect.iter().nth(2).unwrap();
                if (source_start..source_start + source_increment).contains(&humidity_val) {
                    return accum + dest_start + humidity_val - source_start;
                }
                accum
            });
            let loc_val = match loc_val {
                0 => humidity_val,
                _ => loc_val,
            };
            loc_val
        });

    a.min().unwrap()
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Cannot read file");
    println!("part1: {:?}", part1(&input));
    println!("part2 {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_dev_part1() {
        let input = fs::read_to_string("./input.dev.txt").expect("Cannot read file");
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_dev_part2() {
        let input = fs::read_to_string("./input.dev.txt").expect("Cannot read file");
        assert_eq!(part2(&input), 46);
    }
}
