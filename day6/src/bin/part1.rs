use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{multispace0, space1, u32};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

use nom_supreme::parser_ext::ParserExt;

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn calc_ways(&self) -> usize {
        let speed_increase = 1;

        (0..self.time + 1)
            .map(|t_hold| {
                let speed = t_hold * speed_increase;
                speed * (self.time - t_hold)
            })
            .filter(|d| d > &self.distance)
            .count()
    }
}

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    let (_, (time, distance)) = parse(input).expect("should parse");
    let races: Vec<_> = time
        .iter()
        .zip(distance.iter())
        .map(|(t, d)| Race {
            time: *t,
            distance: *d,
        })
        .collect();
    races
        .iter()
        .map(|r| r.calc_ways())
        .fold(1, |acc, r| acc * r)
        .to_string()
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, (time, distance)) =
        separated_pair(parse_line, multispace0, parse_line).parse(input)?;
    Ok((input, (time, distance)))
}

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, items) = take_until(":")
        .precedes(tag(":"))
        .precedes(multispace0)
        .precedes(separated_list0(space1, u32))
        .parse(input)?;
    Ok((input, items))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input: &str = "Time:      7  15   30
Distance:  9  40  200";
        let result: String = part1(test_input);
        assert_eq!(result, "288".to_string());
    }
}
