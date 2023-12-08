use std::collections::BTreeMap;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric0, multispace0},
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
    let result: String = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> String {
    let (_, (mut instructions, nodes)) = parse(input).expect("Failed to parse input");
    let all_nodes = nodes.keys().cloned().collect::<Vec<_>>();

    let a_nodes = all_nodes
        .iter()
        .filter(|x| x.ends_with("A"))
        .collect::<Vec<_>>();

    let intervals = a_nodes
        .iter()
        .map(|x| {
            let mut c = 0;
            let mut curr = **x;
            while !curr.ends_with("Z") {
                let (left, right) = nodes.get(curr).unwrap();
                let next_inst = instructions.get_next().unwrap();
                if next_inst == 'R' {
                    curr = *right;
                } else {
                    curr = *left;
                }
                c += 1;
            }
            c
        })
        .collect::<Vec<_>>();

    get_lcm(intervals).to_string()
}

fn get_lcm(intervals: Vec<i32>) -> i64 {
    let largest = intervals.iter().max().unwrap();
    let mut curr = *largest as i64;
    loop {
        if intervals.iter().all(|x| (curr % (*x as i64) == 0)) {
            break;
        }

        curr += *largest as i64;
    }

    curr
}

fn parse(input: &str) -> IResult<&str, (Instruction, BTreeMap<&str, (&str, &str)>)> {
    let (input, (instructions_str, nodes)) =
        separated_pair(alphanumeric0, multispace0, parse_nodes)(input)?;
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
    let (input, children) = separated_pair(alphanumeric0, tag(", "), alphanumeric0)(input)?;
    let (input, _) = preceded(tag(")"), alphanumeric0)(input)?;
    Ok((input, (key, children)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result: String = part2(test_input);
        assert_eq!(result, "6".to_string());
    }
}
