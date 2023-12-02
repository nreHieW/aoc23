fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    let items = input.split("\n");
    let mut sum: u32 = 0;
    for item in items {
        for i in item.chars() {
            if i.is_digit(10) {
                sum += i.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for i in item.chars().rev() {
            if i.is_digit(10) {
                sum += i.to_digit(10).unwrap();
                break;
            }
        }
    }
    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input: &str = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
        ";
        let result: String = part1(test_input);
        assert_eq!(result, "142".to_string());
    }
}
