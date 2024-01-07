use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn get_attr(&self, attr: char) -> i32 {
        match attr {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Invalid attribute"),
        }
    }

    fn sum_attrs(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

fn main() {
    let input: &str = include_str!("input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    let (workflows, parts) = parse(input);

    let mut rejected = Vec::new();
    let mut accepted = Vec::new();

    parts.iter().for_each(|p| {
        let mut curr_ptr = workflows.get("in").expect("No in workflow");
        let mut out = String::new();
        while (out != "R") || (out != "A") {
            for w in curr_ptr {
                out = w(*p);
                if out == "R" || out == "A" {
                    break;
                }
                if out != "NA" {
                    curr_ptr = workflows
                        .get(&out)
                        .expect(format!("No {} workflow", out).as_str());
                    break;
                }
            }
            if out == "R" || out == "A" {
                break;
            }
        }
        if out == "A" {
            accepted.push(p);
        } else if out == "R" {
            rejected.push(p);
        }
    });

    accepted
        .iter()
        .map(|p| p.sum_attrs())
        .sum::<i32>()
        .to_string()
}

fn parse(input: &str) -> (HashMap<String, Vec<Box<dyn Fn(Part) -> String>>>, Vec<Part>) {
    let tmp = input.split("\n\n").collect::<Vec<&str>>();
    let workflows_raw = tmp[0];
    let parts_raw = tmp[1];

    let mut workflows: HashMap<String, Vec<Box<dyn Fn(Part) -> String>>> = HashMap::new();
    workflows_raw.lines().for_each(|line| {
        let line = line.split("{").collect::<Vec<&str>>();
        let name = line[0];
        let line = line[1];
        let line = line.replace("{", "").replace("}", "");
        let rules_raw = line.split(",").collect::<Vec<&str>>();

        let mut rules = Vec::new();
        rules_raw.iter().for_each(|r| {
            let r = (**r).to_string();
            let f: Box<dyn Fn(Part) -> String>;
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
                let op = match cond.chars().nth(1).unwrap() {
                    '<' => |a: i32, b: i32| -> bool { a < b },
                    '>' => |a: i32, b: i32| -> bool { a > b },
                    _ => panic!("Invalid operator"),
                };
                let val = cond[2..].parse::<i32>().unwrap();

                f = Box::new(move |p: Part| -> String {
                    if op(p.get_attr(attr), val) {
                        res.to_string()
                    } else {
                        "NA".to_string()
                    }
                });
            } else {
                // it is the last one
                f = Box::new(move |_p: Part| -> String { r.to_string() });
            }
            rules.push(f);
        });

        workflows.insert(name.to_string(), rules);
    });

    let parts = parts_raw
        .lines()
        .map(|line| {
            let line = line.replace("{", "").replace("}", "");
            let attrs_raw = line.split(",").collect::<Vec<&str>>();

            let mut attrs = HashMap::new();
            attrs_raw.iter().for_each(|a| {
                let tmp = a.split("=").collect::<Vec<&str>>();
                let (attr, val) = (tmp[0], tmp[1]);
                attrs.insert(attr.to_string(), val.parse::<i32>().unwrap());
            });

            Part {
                x: *attrs.get("x").unwrap(),
                m: *attrs.get("m").unwrap(),
                a: *attrs.get("a").unwrap(),
                s: *attrs.get("s").unwrap(),
            }
        })
        .collect::<Vec<Part>>();

    (workflows, parts)
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
        let result: String = part1(test_input);
        assert_eq!(result, "19114".to_string());
    }
}
