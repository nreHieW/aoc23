use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, space1, u64},
    multi::{many1, separated_list0},
    sequence::{preceded, tuple},
    IResult, Parser,
};

use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use std::ops::Range;

use nom_supreme::parser_ext::ParserExt;
use rayon::prelude::*;

#[derive(Debug, Clone)]
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
    let result: String = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> String {
    let (_, (seeds, maps)) = parse(input).unwrap();

    merge_ranges(seeds)
        .iter()
        .flat_map(|s| s.clone())
        .collect::<Vec<_>>()
        .par_iter()
        .progress()
        .map(|s| maps.iter().fold(*s, |s, map| map.get_res(s)))
        .collect::<Vec<u64>>()
        .iter()
        .min()
        .unwrap()
        .to_string()
}

fn merge_ranges(mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut merged_ranges: Vec<Range<u64>> = vec![ranges[0].clone()];
    ranges.iter().skip(1).for_each(|range| {
        let last_merged_range = merged_ranges.last_mut().unwrap();

        if range.start <= last_merged_range.end {
            if range.end > last_merged_range.end {
                last_merged_range.end = range.end;
            }
        } else {
            merged_ranges.push(range.clone());
        }
    });
    merged_ranges
}

fn parse(input: &str) -> IResult<&str, (Vec<Range<u64>>, Vec<Map>)> {
    let (input, (_, init_seeds)) = tuple((
        preceded(tag("seeds:"), space1),
        separated_list0(space1, u64),
    ))(input)?;
    let seeds = get_seeds(init_seeds);
    let (input, maps) = many1(parse_mapping)(input)?;
    Ok((input, (seeds, maps)))
}

fn get_seeds(seeds: Vec<u64>) -> Vec<Range<u64>> {
    seeds
        .iter()
        .chunks(2)
        .into_iter()
        .map(|chunk| {
            let pair = chunk.collect::<Vec<_>>();
            let start = *pair[0];
            let n = *pair[1];
            start..(start + n)
        })
        .collect::<Vec<_>>()
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
        let result: String = part2(test_input);
        assert_eq!(result, "46".to_string());
    }
}
