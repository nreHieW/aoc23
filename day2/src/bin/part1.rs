fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn is_possible(red: u32, green: u32, blue: u32) -> bool {
    return red <= 12 && green <= 13 && blue <= 14;
}

fn parse_game(game_string: &str) -> bool {
    let game = game_string.split(": ").last().unwrap();
    let stages = game.split("; ");
    let out = stages
        .map(|stage| {
            let mut red_count = 0;
            let mut green_count = 0;
            let mut blue_count = 0;

            for count in stage.split(',').map(|x| x.trim()) {
                if count.ends_with("red") {
                    if let Ok(num) = count.trim_matches(|c: char| !c.is_digit(10)).parse::<u32>() {
                        red_count += num;
                    }
                } else if count.ends_with("green") {
                    if let Ok(num) = count.trim_matches(|c: char| !c.is_digit(10)).parse::<u32>() {
                        green_count += num;
                    }
                } else if count.ends_with("blue") {
                    if let Ok(num) = count.trim_matches(|c: char| !c.is_digit(10)).parse::<u32>() {
                        blue_count += num;
                    }
                }
            }
            return is_possible(red_count, green_count, blue_count);
        })
        .collect::<Vec<bool>>()
        .iter()
        .all(|&x| x);
    return out;
}

fn part1(input: &str) -> String {
    let games = input.trim().split("\n");
    let result = games
        .enumerate()
        .map(|(i, game)| {
            let is_possible = parse_game(game);
            if is_possible {
                return i + 1;
            } else {
                return 0;
            }
        })
        .sum::<usize>()
        .to_string();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input: &str = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";
        let result: String = part1(test_input);
        assert_eq!(result, "8".to_string());
    }
}
