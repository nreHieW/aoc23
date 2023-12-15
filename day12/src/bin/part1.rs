fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Condition {
    DAMAGED,
    OPERATIONAL,
    UNKNOWN,
}

#[derive(Debug, Clone, Copy)]
struct Spring {
    condition: Condition,
}

#[derive(Debug, Clone)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<i32>,
}

impl Record {
    fn permute(&self) -> Vec<Record> {
        use Condition::*;

        let mut unknown_indices = Vec::new();

        // Find indices of unknown conditions
        for (i, spring) in self.springs.iter().enumerate() {
            if spring.condition == UNKNOWN {
                unknown_indices.push(i);
            }
        }

        let num_unknowns = unknown_indices.len();
        let combinations = 1 << num_unknowns; // 2^K combinations
        let mut permutations = Vec::with_capacity(combinations);

        // Generate combinations of unknown conditions
        for i in 0..combinations {
            let mut temp_record = self.clone();
            for j in 0..num_unknowns {
                let bit = 1 << j;
                let index = unknown_indices[j];
                if i & bit != 0 {
                    temp_record.springs[index].condition = OPERATIONAL;
                } else {
                    temp_record.springs[index].condition = DAMAGED;
                }
            }
            permutations.push(temp_record);
        }

        permutations
    }

    fn check(self) -> bool {
        let string_repr = self.string_repr();
        let binding = string_repr.split(".").collect::<Vec<&str>>();
        let combined_springs = binding
            .iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<&&str>>();
        let num_grps = combined_springs.len();
        if num_grps != self.groups.len() {
            return false;
        }

        combined_springs
            .iter()
            .enumerate()
            .all(|(i, c)| c.len() == self.groups[i] as usize)
    }

    fn string_repr(&self) -> String {
        let mut res = String::new();
        self.springs.iter().for_each(|s| {
            res.push(match s.condition {
                Condition::UNKNOWN => '?',
                Condition::OPERATIONAL => '.',
                Condition::DAMAGED => '#',
            });
        });
        res
    }
}

fn part1(input: &str) -> String {
    let mut records = parse(input);
    records
        .iter_mut()
        .flat_map(|r| r.permute())
        .collect::<Vec<Record>>()
        .iter()
        .filter(|r| (*r).clone().check())
        .collect::<Vec<&Record>>()
        .len()
        .to_string()
}

fn parse(input: &str) -> Vec<Record> {
    let mut res = Vec::new();

    input.lines().for_each(|line| {
        let split: Vec<&str> = line.split(" ").collect();
        let springs = split[0];
        let groups = split[1];
        let mut spring_vec = Vec::new();
        springs.chars().for_each(|c| {
            spring_vec.push(Spring {
                condition: match c {
                    '?' => Condition::UNKNOWN,
                    '.' => Condition::OPERATIONAL,
                    '#' => Condition::DAMAGED,
                    _ => panic!("Unknown condition"),
                },
            });
        });

        let mut group_vec = Vec::new();
        groups.split(",").for_each(|g| {
            group_vec.push(g.parse::<i32>().unwrap());
        });

        res.push(Record {
            springs: spring_vec,
            groups: group_vec,
        });
    });

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result: String = part1(test_input);
        assert_eq!(result, "21".to_string());
    }
}
