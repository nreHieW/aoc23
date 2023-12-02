fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part2(input);
    println!("Result: {}", result);
}

fn parse_game(game_string: &str) -> u32 {
    let game = game_string.split(": ").last().unwrap();
    let stages = game.split("; ");

    let mut biggest_red = 0;
    let mut biggest_green = 0;
    let mut biggest_blue = 0;

    stages.for_each(|stage| {
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

        biggest_red = red_count.max(biggest_red);
        biggest_green = green_count.max(biggest_green);
        biggest_blue = blue_count.max(biggest_blue);
    });
    return biggest_red * biggest_green * biggest_blue;
}

fn part2(input: &str) -> String {
    let games = input.trim().split("\n");
    let result = games.map(|game| parse_game(game)).sum::<u32>().to_string();
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
        let result: String = part2(test_input);
        assert_eq!(result, "2286".to_string());
    }
}
