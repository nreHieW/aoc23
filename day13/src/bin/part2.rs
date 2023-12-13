use core::panic;
use itertools::Itertools;
fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part2(input);
    println!("Result: {}", result);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Puzzle {
    grid_as_rows: Vec<Vec<char>>,
    grid_as_cols: Vec<Vec<char>>,
    num_rows: i32,
    num_cols: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ReflectionLine {
    idx: i32,
    orientation: Orientation,
}

impl Puzzle {
    fn swap(val: char) -> char {
        match val {
            '#' => '.',
            '.' => '#',
            _ => panic!("Invalid value"),
        }
    }

    fn generate_variants(&self) -> Vec<Puzzle> {
        let mut variants = Vec::new();
        for x in 0..self.num_rows {
            for y in 0..self.num_cols {
                let mut grid_as_rows = self.grid_as_rows.clone();
                let mut grid_as_cols = self.grid_as_cols.clone();
                grid_as_rows[x as usize][y as usize] =
                    Puzzle::swap(grid_as_rows[x as usize][y as usize]);
                grid_as_cols[y as usize][x as usize] =
                    Puzzle::swap(grid_as_cols[y as usize][x as usize]);
                variants.push(Puzzle {
                    grid_as_rows: grid_as_rows,
                    grid_as_cols: grid_as_cols,
                    num_rows: self.num_rows,
                    num_cols: self.num_cols,
                });
            }
        }
        variants
    }

    fn find_reflection(&self) -> Vec<ReflectionLine> {
        let mut res = Vec::new();
        for i in 1..self.num_rows {
            // i represents the start of the right side
            let mut left_indices = 0..i;
            let mut right_indices = i..self.num_rows;
            if left_indices.len() > right_indices.len() {
                let diff = left_indices.len() - right_indices.len();
                left_indices = diff as i32..i;
            }

            if right_indices.len() > left_indices.len() {
                let diff = right_indices.len() - left_indices.len();
                right_indices = i..(self.num_rows - diff as i32);
            }

            let left = left_indices
                .map(|x| self.grid_as_rows[x as usize].clone())
                .collect::<Vec<_>>();
            let mut right = right_indices
                .map(|x| self.grid_as_rows[x as usize].clone())
                .collect::<Vec<_>>();
            right.reverse();
            if left == right && left.len() > 0 {
                res.push(ReflectionLine {
                    idx: i,
                    orientation: Orientation::Horizontal,
                });
            }
        }

        for i in 1..self.num_cols {
            // i represents the start of the right side
            let mut left_indices = 0..i;
            let mut right_indices = i..self.num_cols;
            if left_indices.len() > right_indices.len() {
                let diff = left_indices.len() - right_indices.len();
                left_indices = diff as i32..i;
            }

            if right_indices.len() > left_indices.len() {
                let diff = right_indices.len() - left_indices.len();
                right_indices = i..(self.num_cols - diff as i32);
            }

            let left = left_indices
                .map(|x| self.grid_as_cols[x as usize].clone())
                .collect::<Vec<_>>();
            let mut right = right_indices
                .map(|x| self.grid_as_cols[x as usize].clone())
                .collect::<Vec<_>>();

            right.reverse();
            if (left == right) && left.len() > 0 {
                res.push(ReflectionLine {
                    idx: i,
                    orientation: Orientation::Vertical,
                });
            }
        }
        res
    }
}

fn part2(input: &str) -> String {
    let puzzles = parse(input);
    let mut cols = Vec::new();
    let mut rows = Vec::new();
    puzzles.iter().for_each(|x| {
        let orig = x.find_reflection();
        let variants = x.generate_variants();
        let item: Vec<_> = variants
            .iter()
            .flat_map(|x| x.find_reflection())
            .filter(|x| !orig.contains(x))
            .unique()
            .collect::<Vec<_>>();
        if item.len() != 1 {
            println!("item {:?}", item);
            panic!("Invalid reflection");
        }
        if item[0].orientation == Orientation::Horizontal {
            rows.push(item[0].idx);
        } else {
            cols.push(item[0].idx);
        }
    });
    println!("Rows: {:?}", rows);
    println!("Cols: {:?}", cols);
    (cols.iter().sum::<i32>() + 100 * rows.iter().sum::<i32>()).to_string()
}

fn parse(input: &str) -> Vec<Puzzle> {
    input
        .split("\n\n")
        .map(|x| {
            let mut grid_as_rows = Vec::new();
            x.lines().for_each(|y| {
                grid_as_rows.push(y.chars().collect::<Vec<char>>());
            });
            let num_rows = grid_as_rows.len();
            let num_cols = grid_as_rows[0].len();

            let mut grid_as_cols = Vec::new();
            for i in 0..num_cols {
                let mut col = Vec::new();
                for j in 0..num_rows {
                    col.push(grid_as_rows[j][i]);
                }
                grid_as_cols.push(col);
            }
            Puzzle {
                grid_as_rows: grid_as_rows,
                grid_as_cols: grid_as_cols,
                num_rows: num_rows as i32,
                num_cols: num_cols as i32,
            }
        })
        .collect::<Vec<_>>()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        //         let test_input = "..#.##.
        // ..####.
        // ..####.
        // ..#.##.
        // ##....#
        // .#.##.#
        // #.#..#.";
        let result: String = part2(test_input);
        assert_eq!(result, "400".to_string());
    }
}
