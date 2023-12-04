use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, line_ending, space1, u32},
    multi::separated_list0,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
struct Card {
    _id: usize,
    winning: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn get_points(&self) -> usize {
        let num_wins = self
            .numbers
            .iter()
            .filter(|x| self.winning.contains(x))
            .count();
        if num_wins == 0 {
            return 0;
        }
        return 2_usize.pow((num_wins - 1) as u32);
    }
}

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    let cards = parse_cards(input).expect("Should parse cards");
    return cards
        .1
        .iter()
        .map(|x| x.get_points())
        .sum::<usize>()
        .to_string();
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list0(line_ending, card)(input)?;
    return Ok((input, cards));
}

// Parse a list of numbers
fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, numbers) = separated_list0(space1, u32)(input)?;
    return Ok((input, numbers));
}

// Split on |
fn split_winning(input: &str) -> IResult<&str, &str> {
    let (input, part1) = take_until("|")(input)?;
    let (part2, _) = tag("|")(input)?;

    return Ok((part1.trim(), part2.trim()));
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, _) = preceded(tag("Card"), space1)(input)?;
    let (input, id) = digit1(input)?;
    let (part1, part2) = preceded(tag(": "), split_winning)(input)?;
    let (_, winning) = parse_numbers(part1)?;
    let (part2, numbers) = parse_numbers(part2)?;
    return Ok((
        part2,
        Card {
            _id: id.parse::<usize>().unwrap(),
            winning: winning.iter().map(|x| *x as usize).collect(),
            numbers: numbers.iter().map(|x| *x as usize).collect(),
        },
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input: &str = "Card  1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result: String = part1(test_input);
        assert_eq!(result, "13".to_string());
    }
}
