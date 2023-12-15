#[derive(Debug)]
struct RoundedRock {
    y: usize,
}

impl RoundedRock {
    fn move_rock(&mut self, y: usize) {
        self.y = y;
    }

    fn calc_load(&self, height: usize) -> usize {
        height - self.y
    }
}

#[derive(Debug)]
struct RockColumn {
    cube_rocks_locations: Vec<usize>,
    rounded_rocks: Vec<RoundedRock>, // sorted by y
}

impl RockColumn {
    fn process(&mut self) {
        let num_rocks = self.rounded_rocks.len();
        for i in 0..num_rocks {
            let largest_cube_rock = self
                .cube_rocks_locations
                .iter()
                .filter(|y| **y < self.rounded_rocks[i].y)
                .max()
                .map(|x| *x as i32)
                .unwrap_or(-1);
            let largest_rounded_rock = self
                .rounded_rocks
                .iter()
                .filter(|r| (**r).y < self.rounded_rocks[i].y)
                .map(|r| r.y)
                .max()
                .map(|x| x as i32)
                .unwrap_or(-1);

            let idx = largest_cube_rock.max(largest_rounded_rock);
            if idx == -1 {
                self.rounded_rocks[i].move_rock(0);
            } else {
                self.rounded_rocks[i].move_rock(idx as usize + 1);
            }
        }
    }
}

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}
fn part1(input: &str) -> String {
    let (mut rock_columns, height) = parse(input);
    // for all rock sorted by y: new location of rounded rock = min(largest idx of cube rocks > curr, largest idx of rounded rock > curr) - 1
    rock_columns.iter_mut().for_each(|item| item.process());
    rock_columns
        .iter()
        .map(|col| {
            col.rounded_rocks
                .iter()
                .map(|x| x.calc_load(height))
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn parse(input: &str) -> (Vec<RockColumn>, usize) {
    let all_chars = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let num_cols = input.lines().next().unwrap().len();
    let num_rows = input.lines().count();

    let mut rock_columns: Vec<RockColumn> = Vec::new();
    for x in 0..num_cols {
        let mut curr_rounded_rocks: Vec<RoundedRock> = Vec::new();
        let mut curr_cube_rocks_locations: Vec<usize> = Vec::new();
        for y in 0..num_rows {
            let curr = all_chars[y][x];
            if curr == '#' {
                curr_cube_rocks_locations.push(y);
            } else if curr == 'O' {
                curr_rounded_rocks.push(RoundedRock { y });
            }
        }
        rock_columns.push(RockColumn {
            cube_rocks_locations: curr_cube_rocks_locations,
            rounded_rocks: curr_rounded_rocks,
        });
    }
    (rock_columns, num_rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let result: String = part1(test_input);
        assert_eq!(result, "136".to_string());
    }
}
