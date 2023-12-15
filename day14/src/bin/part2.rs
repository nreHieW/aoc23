use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RockType {
    Cube,
    Rounded,
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rock {
    x: usize,
    y: usize,
    rock_type: RockType,
}

impl Rock {
    fn move_rock(&mut self, y: usize) {
        self.y = y;
    }

    fn calc_load(&self, height: usize) -> usize {
        height - self.y
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rock_type = match self.rock_type {
            RockType::Cube => "#",
            RockType::Rounded => "O",
            RockType::Null => ".",
        };
        write!(f, "{}", rock_type)
    }
}

#[derive(Debug)]
struct RockColumn {
    x: usize,
    cube_rocks_locations: Vec<usize>,
    rounded_rocks: Vec<Rock>, // sorted by y
    cube_rocks: Vec<Rock>,
    empty_rocks: Vec<Rock>,
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
    let result: String = part2(input);
    println!("Result: {}", result);
}
fn part2(input: &str) -> String {
    let (mut rocks, height, width) = parse(input);
    let mut early_stop = 0;
    let mut cache = HashMap::new();
    let mut cycle_len = 0;
    for i in 1..=1000000000u32 {
        let string_grid = get_string_grid(rocks.clone());
        if cache.contains_key(&string_grid) {
            early_stop = i;
            let prev = cache.get(&string_grid).unwrap();
            cycle_len = i - prev;
            break;
        }
        for _ in 0..4 {
            let mut cols = generate_columns(rocks.clone(), height, width);
            cols.iter_mut().for_each(|col| col.process());
            rocks = collate(cols, height, width);
            rotate_matrix(&mut rocks, height, width);
        }
        cache.insert(string_grid, i);
    }

    let remainder = (1000000000u32 - early_stop) % cycle_len as u32;
    for _ in 0..remainder + 1 {
        for _ in 0..4 {
            let mut cols = generate_columns(rocks.clone(), height, width);
            cols.iter_mut().for_each(|col| col.process());
            rocks = collate(cols, height, width);
            rotate_matrix(&mut rocks, height, width);
        }
    }
    print_grid(rocks.clone());

    let height = rocks.len();
    generate_columns(rocks, height, width)
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

fn print_grid(rocks: Vec<Vec<Rock>>) {
    for row in rocks {
        for rock in row {
            print!("{}", rock);
        }
        println!();
    }
}

fn get_string_grid(rocks: Vec<Vec<Rock>>) -> String {
    let mut res = String::new();
    for row in rocks {
        for rock in row {
            res.push_str(&format!("{}", rock));
        }
        res.push_str("\n");
    }
    res
}

fn collate(cols: Vec<RockColumn>, height: usize, width: usize) -> Vec<Vec<Rock>> {
    let mut rocks = Vec::new();
    for col in cols {
        let total_rocks = col.rounded_rocks.len() + col.cube_rocks.len() + col.empty_rocks.len();
        let mut col_rocks = Vec::new();
        for rock in col.rounded_rocks {
            col_rocks.push(rock);
        }
        for rock in col.cube_rocks {
            col_rocks.push(rock);
        }
        let remaining_ys = (0..total_rocks)
            .filter(|x| !col_rocks.iter().any(|r| r.y == *x))
            .collect::<Vec<_>>();
        for item in remaining_ys {
            col_rocks.push(Rock {
                x: col.x,
                y: item,
                rock_type: RockType::Null,
            });
        }
        col_rocks.sort_by(|a, b| a.y.cmp(&b.y));
        rocks.push(col_rocks);
    }
    let mut res = Vec::new();
    for x in 0..width {
        let mut row = Vec::new();
        for y in 0..height {
            row.push(rocks[y][x]);
        }
        res.push(row);
    }
    res
}

fn generate_columns(rocks: Vec<Vec<Rock>>, height: usize, width: usize) -> Vec<RockColumn> {
    let mut cols = Vec::new();

    for i in 0..width {
        let mut cube_rocks_locations = Vec::new();
        let mut rounded_rocks = Vec::new();
        let mut cube_rocks = Vec::new();
        let mut empty_rocks = Vec::new();
        for j in 0..height {
            let rock = rocks[j][i];
            match rock.rock_type {
                RockType::Cube => {
                    cube_rocks_locations.push(rock.y);
                    cube_rocks.push(rock)
                }
                RockType::Rounded => rounded_rocks.push(rock),
                RockType::Null => empty_rocks.push(rock),
            }
        }
        cols.push(RockColumn {
            x: i,
            cube_rocks_locations,
            rounded_rocks,
            cube_rocks,
            empty_rocks,
        });
    }
    cols
}

fn rotate_matrix(matrix: &mut Vec<Vec<Rock>>, height: usize, width: usize) {
    let mut transposed_matrix = Vec::new();
    for i in 0..width {
        let mut row = Vec::new();
        for j in 0..height {
            row.push(matrix[j][i]);
        }
        transposed_matrix.push(row);
    }

    // reverse each row
    transposed_matrix = transposed_matrix
        .iter()
        .map(|row| row.iter().rev().cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // set coordinates
    for i in 0..width {
        for j in 0..height {
            transposed_matrix[i][j].x = j;
            transposed_matrix[i][j].y = i;
        }
    }

    *matrix = transposed_matrix;
}

fn parse(input: &str) -> (Vec<Vec<Rock>>, usize, usize) {
    let all_chars = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let num_cols = input.lines().next().unwrap().len();
    let num_rows = input.lines().count();

    let mut all_rocks = Vec::new();
    for i in 0..num_rows {
        let mut row = Vec::new();
        for j in 0..num_cols {
            let rock_type = match all_chars[i][j] {
                '#' => RockType::Cube,
                'O' => RockType::Rounded,
                '.' => RockType::Null,
                _ => panic!("Invalid rock type"),
            };
            row.push(Rock {
                x: j,
                y: i,
                rock_type,
            });
        }
        all_rocks.push(row);
    }
    (all_rocks, num_rows, num_cols)
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
        let result: String = part2(test_input);
        assert_eq!(result, "64".to_string());
    }
}
