use std::{collections::HashMap, iter::once};

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part2(input);
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    index: i32,
    group_index: i32,
    chunk_size: i32,
}

fn part2(input: &str) -> String {
    let records = parse(input);
    let dp: HashMap<State, i64> = HashMap::new();

    // State is current index, current group index, current chunk size
    // dp[state] = number of valid permutations
    records
        .iter()
        .map(|x| process(&mut dp.clone(), x.clone(), 0, 0, 0))
        .sum::<i64>()
        .to_string()
}

fn process(dp: &mut HashMap<State, i64>, curr: Record, idx: i32, grp: i32, chunk_size: i32) -> i64 {
    use Condition::*;
    let state = State {
        index: idx,
        group_index: grp,
        chunk_size: chunk_size,
    };
    if dp.contains_key(&state) {
        return (*dp)[&state] as i64;
    }

    if (grp >= curr.groups.len() as i32) && (chunk_size != 0) {
        return 0;
    }

    let curr_grp = if grp >= curr.groups.len() as i32 {
        -1
    } else {
        curr.groups[grp as usize]
    };
    // base cases
    if idx >= curr.springs.len() as i32 {
        if (chunk_size == curr_grp) && (grp == (curr.groups.len() - 1) as i32) {
            return 1;
        } else if (grp >= curr.groups.len() as i32) && (chunk_size == 0) {
            return 1;
        } else {
            return 0;
        }
    }
    let curr_spring = curr.springs[idx as usize];
    let mut res;
    if curr_spring.condition == DAMAGED {
        // 2 cases: either continue an existing chunk or start a new chunk
        if chunk_size > 0 {
            // We were inside a chunk so we continue (eg. ######)
            res = process(dp, curr, idx + 1, grp, chunk_size + 1);
        } else {
            // We are not inside a chunk so we start a new chunk (eg. ###...)
            res = process(dp, curr, idx + 1, grp, 1);
        }
    } else if curr_spring.condition == OPERATIONAL {
        if chunk_size > 0 {
            // We were inside a chunk so we end the chunk (eg. ###...)
            if chunk_size != curr_grp {
                // this chunk does not match the group size
                res = 0;
            } else {
                // valid chunk
                res = process(dp, curr, idx + 1, grp + 1, 0);
            }
        } else {
            // We are not inside a chunk so we continue (eg. ....)
            res = process(dp, curr, idx + 1, grp, 0);
        }
    } else if curr_spring.condition == UNKNOWN {
        // 2 cases add together either treat as operational or damaged
        if chunk_size > 0 {
            res = process(dp, curr.clone(), idx + 1, grp, chunk_size + 1);
            if chunk_size == curr_grp {
                res += process(dp, curr.clone(), idx + 1, grp + 1, 0);
            }
        } else {
            res = process(dp, curr.clone(), idx + 1, grp, 0)
                + process(dp, curr.clone(), idx + 1, grp, 1);
        }
    } else {
        panic!("Unknown condition");
    }
    dp.insert(state, res);
    res
}

fn parse(input: &str) -> Vec<Record> {
    let mut res = Vec::new();

    input.lines().for_each(|line| {
        let split: Vec<&str> = line.split(" ").collect();
        let springs = split[0];
        let groups = split[1];
        let mut spring_vec = Vec::new();
        once(springs)
            .cycle()
            .take(5)
            .collect::<Vec<_>>()
            .join("?")
            .chars()
            .for_each(|c| {
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
            groups: group_vec
                .clone()
                .iter()
                .into_iter()
                .cycle()
                .take(5 * group_vec.len())
                .collect::<Vec<_>>()
                .iter()
                .map(|x| **x)
                .collect(),
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
        let result: String = part2(test_input);
        assert_eq!(result, "525152".to_string());
    }
}
