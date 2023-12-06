use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{multispace0, space1, u64};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

use nom_supreme::parser_ext::ParserExt;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
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
    let result: String = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> String {
    let (_, (time, distance)) = parse(input).expect("should parse");
    let race = Race {
        time: time,
        distance: distance,
    };
    race.calc_ways().to_string()
}

fn parse(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, (time, distance)) =
        separated_pair(parse_line, multispace0, parse_line).parse(input)?;
    Ok((input, (time, distance)))
}

fn parse_line(input: &str) -> IResult<&str, u64> {
    let (input, numbers) = take_until(":")
        .precedes(tag(":"))
        .precedes(multispace0)
        .precedes(separated_list0(space1, u64))
        .parse(input)?;
    Ok((
        input,
        numbers.iter().fold(0, |acc, n| {
            let num_digit = (*n as f32).log10() as u32 + 1;
            acc * 10_u64.pow(num_digit) + n
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input: &str = "Time:      7  15   30
Distance:  9  40  200";
        let result: String = part2(test_input);
        assert_eq!(result, "71503".to_string());
    }
}
