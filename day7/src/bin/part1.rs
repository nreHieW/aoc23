use std::{cmp::Ordering, collections::HashMap};

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric0, digit0, line_ending},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

#[derive(Debug)]
struct Card {
    field: char,
}

impl Card {
    fn compare(&self, other: &Card) -> Ordering {
        let orders = [
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ];
        let self_order = orders.iter().position(|&x| x == self.field).unwrap();
        let other_order = orders.iter().position(|&x| x == other.field).unwrap();
        self_order.cmp(&other_order).reverse()
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

impl Hand {
    fn into_character_map(&self, char_cards: Vec<char>) -> HashMap<char, usize> {
        char_cards.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(*c).or_insert(0) += 1;
            acc
        })
    }

    fn score(&self) -> usize {
        let char_map = self.into_character_map(self.cards.iter().map(|c| c.field).collect());
        let counts = char_map.values().collect::<Vec<&usize>>();
        if counts.len() == 5 {
            0
        } else if counts.len() == 4 {
            1
        } else if counts.len() == 3 {
            // 3 unique cards
            if counts.iter().filter(|&&c| *c == 2).count() == 2 {
                // 2 pairs
                2
            } else {
                // 3 of a kind
                3
            }
        } else if counts.len() == 2 {
            // 2 unique cards
            if counts.iter().max().unwrap() == &&3 {
                // full house
                4
            } else {
                // 4 of a kind
                5
            }
        } else {
            // 1 unique card = 5 of a kind
            6
        }
    }

    fn compare(&self, other: &Hand) -> Ordering {
        let self_score = self.score();
        let other_score = other.score();
        if self_score == other_score {
            // compare each card until first difference
            for i in 0..self.cards.len() {
                let self_card = &self.cards[i];
                let other_card = &other.cards[i];
                let card_order = self_card.compare(other_card);
                if card_order != Ordering::Equal {
                    return card_order;
                } else {
                    continue;
                }
            }
            Ordering::Equal
        } else {
            self_score.cmp(&other_score)
        }
    }
}

fn part1(input: &str) -> String {
    let (_, mut hands) = parse(input).expect("Failed to parse input");
    hands.sort_by(|a, b| a.compare(b));
    hands
        .iter()
        .enumerate()
        .map(|(idx, h)| (idx + 1) * h.bid)
        .sum::<usize>()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, parse_hand)(input)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = alphanumeric0(input)?;
    let (input, bid) = preceded(tag(" "), digit0)(input)?;
    Ok((
        input,
        Hand {
            bid: bid.parse::<usize>().unwrap(),
            cards: cards
                .chars()
                .map(|c| Card { field: c })
                .collect::<Vec<Card>>(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result: String = part1(test_input);
        assert_eq!(result, "6440".to_string());
    }
}
