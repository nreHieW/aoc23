use std::{collections::HashMap, ops::Range};

#[derive(Debug, Clone, PartialEq)]
struct Part {
    x: Range<i32>,
    m: Range<i32>,
    a: Range<i32>,
    s: Range<i32>,
}

impl Part {
    fn gen_split(&self, val: i32, attr: char, op: String) -> (Part, Part) {
        // part 1 is the left part, part 2 is the right part
        let mut part1 = self.clone();
        let mut part2 = self.clone();
        match attr {
            'x' => {
                if op == "<" {
                    part1.x = self.x.start..val;
                    part2.x = val..self.x.end;
                } else {
                    part1.x = self.x.start..(val + 1);
                    part2.x = (val + 1)..self.x.end;
                }
            }
            'm' => {
                if op == "<" {
                    part1.m = self.m.start..val;
                    part2.m = val..self.m.end;
                } else {
                    part1.m = self.m.start..(val + 1);
                    part2.m = (val + 1)..self.m.end;
                }
            }
            'a' => {
                if op == "<" {
                    part1.a = self.a.start..val;
                    part2.a = val..self.a.end;
                } else {
                    part1.a = self.a.start..(val + 1);
                    part2.a = (val + 1)..self.a.end;
                }
            }
            's' => {
                if op == "<" {
                    part1.s = self.s.start..val;
                    part2.s = val..self.s.end;
                } else {
                    part1.s = self.s.start..(val + 1);
                    part2.s = (val + 1)..self.s.end;
                }
            }
            _ => panic!("Invalid attribute"),
        }
        (part1, part2)
    }

    fn num_combinations(&self) -> i64 {
        let x = self.x.end - self.x.start;
        let m = self.m.end - self.m.start;
        let a = self.a.end - self.a.start;
        let s = self.s.end - self.s.start;
        (x as i64 * m as i64 * a as i64 * s as i64).max(0)
    }
}

#[derive(Debug, Clone)]
struct Rule {
    attr: char,
    val: i32,
    op: String,
    out: String,
    is_last: bool,
}

type Workflow = Vec<Rule>;

#[derive(Debug)]
struct State {
    part: Part,
    curr_workflow: String,
}

fn main() {
    let input: &str = include_str!("input.txt");
    let result: String = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> String {
    let workflows = parse(input);
    let mut accepted = Vec::new();

    let mut states = Vec::new();
    states.push(State {
        part: Part {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        },
        curr_workflow: "in".to_string(),
    });
    while states.len() != 0 {
        let curr_state = states.remove(0);
        let curr_workflow = workflows
            .get(&curr_state.curr_workflow)
            .expect(format!("No {} workflow", curr_state.curr_workflow).as_str());
        let mut curr = curr_state.part;
        for w in curr_workflow {
            let break_point = w.val;
            let attr = w.attr;
            let op = &w.op;

            let next_state; // next state represents the end of the current workflow

            if w.is_last {
                next_state = State {
                    part: curr.clone(),
                    curr_workflow: w.out.clone(),
                };
            } else {
                let (left_part, right_part) = curr.gen_split(break_point, attr, op.clone());
                if op == "<" {
                    curr = right_part;
                    next_state = State {
                        part: left_part,
                        curr_workflow: w.out.clone(),
                    };
                } else if op == ">" {
                    curr = left_part;
                    next_state = State {
                        part: right_part,
                        curr_workflow: w.out.clone(),
                    };
                } else {
                    panic!("Invalid operator");
                }
            }

            if w.out == "A" {
                accepted.push(next_state.part);
            } else if w.out == "R" {
            } else {
                states.push(next_state);
            }
        }
    }

    accepted
        .iter()
        .map(|p| p.num_combinations())
        .sum::<i64>()
        .to_string()
}

fn parse(input: &str) -> HashMap<String, Workflow> {
    let tmp = input.split("\n\n").collect::<Vec<&str>>();
    let workflows_raw = tmp[0];

    let mut workflows = HashMap::new();
    workflows_raw.lines().for_each(|line| {
        let line = line.split("{").collect::<Vec<&str>>();
        let name = line[0];
        let line = line[1];
        let line = line.replace("{", "").replace("}", "");
        let rules_raw = line.split(",").collect::<Vec<&str>>();
        let mut rules = Vec::new();
        rules_raw.iter().for_each(|r| {
            let r = (**r).to_string();
            if r.contains(":") {
                // it is not the last one
                let tmp = r
                    .split(":")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let (cond, res) = (tmp[0].clone(), tmp[1].clone());

                let attr = cond.chars().nth(0).unwrap();
                let op = cond.chars().nth(1).unwrap().to_string();
                let val = cond[2..].parse::<i32>().unwrap();

                rules.push(Rule {
                    attr,
                    val,
                    op,
                    out: res,
                    is_last: false,
                });
            } else {
                // it is the last one
                rules.push(Rule {
                    attr: 'N',
                    val: -1,
                    op: "".to_string(),
                    out: r,
                    is_last: true,
                });
            }
        });

        workflows.insert(name.to_string(), rules);
    });

    workflows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        let result: String = part2(test_input);
        assert_eq!(result, "167409079868000".to_string());
    }
}
