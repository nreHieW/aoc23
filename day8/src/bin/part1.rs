use std::collections::BTreeMap;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alpha0, multispace0},
    multi::many0,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Instruction {
    orig: Vec<char>,
    current: Vec<char>,
}

impl Instruction {
    fn get_instructions(&mut self) -> Vec<char> {
        if self.current.len() == 0 {
            self.current = self.orig.clone();
            self.current.clone()
        } else {
            self.current.clone()
        }
    }

    fn get_next(&mut self) -> Option<char> {
        let inst = self.get_instructions();
        self.current = inst[1..].to_vec();
        inst.first().cloned()
    }
}

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    let (_, (mut instructions, nodes)) = parse(input).expect("Failed to parse input");
    let mut c = 0;
    let mut curr = "AAA";
    while curr != "ZZZ" {
        let (left, right) = nodes.get(curr).unwrap();
        let next_inst = instructions.get_next().unwrap();
        if next_inst == 'R' {
            curr = *right;
        } else {
            curr = *left;
        }
        c += 1;
    }
    c.to_string()
}

fn parse(input: &str) -> IResult<&str, (Instruction, BTreeMap<&str, (&str, &str)>)> {
    let (input, (instructions_str, nodes)) =
        separated_pair(alpha0, multispace0, parse_nodes)(input)?;
    let instructions_indiv = instructions_str.chars().collect::<Vec<_>>();
    let instructions = Instruction {
        orig: instructions_indiv.clone(),
        current: instructions_indiv,
    };
    Ok((input, (instructions, nodes)))
}

fn parse_nodes(input: &str) -> IResult<&str, BTreeMap<&str, (&str, &str)>> {
    let (input, nodes) = many0(parse_node)(input).expect("Failed to parse nodes");
    let mut nodes_map = BTreeMap::new();
    nodes.iter().for_each(|(key, value)| {
        nodes_map.insert(*key, *value);
    });
    Ok((input, nodes_map))
}

fn parse_node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, key) = take_until(" = ")(input.trim())?;
    let (input, _) = tag(" = (")(input)?;
    let (input, children) = separated_pair(alpha0, tag(", "), alpha0)(input)?;
    let (input, _) = preceded(tag(")"), alpha0)(input)?;
    Ok((input, (key, children)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result: String = part1(test_input);
        assert_eq!(result, "2".to_string());
    }
    #[test]
    fn test2() {
        let test_input: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result: String = part1(test_input);
        assert_eq!(result, "6".to_string());
    }
}
