fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}
fn part1(input: &str) -> String {
    let binding = input.replace("\n", "");
    let segments: Vec<&str> = binding.split(',').collect();
    segments
        .iter()
        .map(|s| hash_algo(s))
        .sum::<i32>()
        .to_string()
}

fn hash_algo(input: &str) -> i32 {
    let mut curr = 0;
    input.chars().for_each(|c| {
        let ascii = c as i32;
        curr += ascii;
        curr *= 17;
        curr = curr % 256;
    });
    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result: String = part1(test_input);
        assert_eq!(result, "1320".to_string());
    }
    #[test]
    fn test2() {
        let test_input: &str = "HASH";
        let result = hash_algo(test_input);
        assert_eq!(result, 52);
    }
}
