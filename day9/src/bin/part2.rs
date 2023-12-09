#[derive(Debug)]
struct History {
    vals: Vec<Vec<i32>>,
}

impl History {
    fn get_next_sequence(&mut self) -> Vec<i32> {
        let last_seq = self.vals.last().expect("Should not be empty");
        let res = last_seq
            .windows(2)
            .map(|item| item[1] - item[0])
            .collect::<Vec<i32>>();
        self.vals.push(res.clone());
        res
    }

    fn process(&mut self) {
        let mut last_seq = self.vals.last_mut().expect("Should not be empty").clone();
        while !last_seq.iter().all(|&x| x == 0) {
            last_seq = self.get_next_sequence();
        }

        let mut curr = 0;
        self.vals.last_mut().unwrap().push(curr);
        self.vals = self
            .vals
            .iter()
            .rev()
            .map(|v| {
                let first_item = *v.first().expect("Should not be empty");
                let mut res = v.clone();
                curr = first_item - curr;
                res.insert(0, curr);
                res
            })
            .collect::<Vec<Vec<i32>>>()
            .iter()
            .rev()
            .map(|v| v.clone())
            .collect();
    }

    fn get_extrapolated(&self) -> i32 {
        *self.vals[0].first().expect("Should not be empty")
    }
}

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> String {
    let mut seqs = parse(input);
    seqs.iter_mut().for_each(|seq| seq.process());
    seqs.iter()
        .map(|seq| seq.get_extrapolated())
        .sum::<i32>()
        .to_string()
}

fn parse(input: &str) -> Vec<History> {
    input
        .lines()
        .map(|line| {
            let vals: Vec<i32> = line
                .split(" ")
                .map(|val| val.parse::<i32>().unwrap())
                .collect();
            History { vals: vec![vals] }
        })
        .collect::<Vec<History>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result: String = part2(test_input);
        assert_eq!(result, "2".to_string());
    }
}
