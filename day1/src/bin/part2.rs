fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part2(input);
    println!("Result: {}", result);
}

fn word_to_num(word: &str) -> Option<u32> {
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            Some(word.parse::<u32>().unwrap())
        }
        _ => None,
    }
}

fn part2(input: &str) -> String {
    let items = input.split("\n");
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum: u32 = 0;
    for item in items {
        let mut curr = Vec::new();
        for (index, i) in item.chars().enumerate() {
            if i.is_digit(10) {
                curr.push([i.to_digit(10).unwrap(), index.try_into().unwrap()]);
            }
        }
        for num in numbers {
            if item.contains(num) {
                let indices = item.match_indices(num);
                for (index, _) in indices {
                    curr.push([word_to_num(num).unwrap(), index.try_into().unwrap()]);
                }
            }
        }

        if curr.len() == 0 {
            continue;
        }
        curr.sort_by(|a, b| a[1].cmp(&b[1]));
        sum += curr[0][0] * 10 + curr.last().unwrap()[0];
    }
    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input: &str = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
        ";
        let result: String = part2(test_input);
        assert_eq!(result, "281".to_string());
    }
}
