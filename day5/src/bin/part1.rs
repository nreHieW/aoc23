use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, space1, u64},
    multi::{many1, separated_list0},
    sequence::{preceded, tuple},
    IResult, Parser,
};

use std::ops::Range;

use nom_supreme::parser_ext::ParserExt;

#[derive(Debug)]
struct Map {
    maps: Vec<Vec<Range<u64>>>,
}

impl Map {
    fn get_res(&self, input: u64) -> u64 {
        let mut res = 0;
        let mut found = false;
        self.maps
            .iter()
            .filter(|m| m[0].contains(&input))
            .collect::<Vec<_>>()
            .iter()
            .for_each(|m| {
                found = true;
                let increment = input - m[0].start;
                res = m[1].start + increment;
            });
        if found {
            res
        } else {
            input
        }
    }
}

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    let (_, (seeds, maps)) = parse(input).unwrap();
    seeds
        .iter()
        .map(|s| maps.iter().fold(*s, |s, map| map.get_res(s)))
        .collect::<Vec<u64>>()
        .iter()
        .min()
        .unwrap()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<Map>)> {
    let (input, (_, seeds)) = tuple((
        preceded(tag("seeds:"), space1),
        separated_list0(space1, u64),
    ))(input)?;
    let (input, maps) = many1(parse_mapping)(input)?;
    Ok((input, (seeds, maps)))
}

fn parse_mapping(input: &str) -> IResult<&str, Map> {
    let (input, mapping) = take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(parse_line)).map(|m| Map { maps: m }))
        .parse(input)?;
    Ok((input, mapping))
}

fn parse_line(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    // println!("parse_line: {}", input);
    let (input, (destination, source, length)) =
        tuple((u64, u64.preceded_by(tag(" ")), u64.preceded_by(tag(" "))))(input)?;
    Ok((
        input,
        vec![
            source..(source + length),
            destination..(destination + length),
        ],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result: String = part1(test_input);
        assert_eq!(result, "35".to_string());
    }
}
