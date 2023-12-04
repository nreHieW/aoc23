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
    num_instances: usize,
}

impl Card {
    fn get_matches(&self) -> usize {
        let num_wins = self
            .numbers
            .iter()
            .filter(|x| self.winning.contains(x))
            .count();
        return num_wins;
    }

    fn add_instance(&mut self) {
        self.num_instances += 1;
    }
}

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    let mut cards = parse_cards(input).expect("Should parse cards");
    let mut sum = 0;
    let card_count = cards.1.len();

    for i in 0..card_count {
        let num_matches = cards.1[i].get_matches();
        for j in 1..num_matches + 1 {
            for _ in 0..cards.1[i].num_instances {
                cards.1[i + j].add_instance();
            }
        }
        sum += cards.1[i].num_instances;
    }

    return sum.to_string();
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
            num_instances: 1,
        },
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result: String = part1(test_input);
        assert_eq!(result, "30".to_string());
    }
}
