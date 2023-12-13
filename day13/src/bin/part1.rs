fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

struct Puzzle {
    grid_as_rows: Vec<Vec<char>>,
    grid_as_cols: Vec<Vec<char>>,
    num_rows: i32,
    num_cols: i32,
}

impl Puzzle {
    fn find_reflection(&self) -> (i32, i32) {
        let mut row_reflect = -1;
        let mut col_reflect = -1;
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
                row_reflect = i;
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
                col_reflect = i;
            }
        }

        (row_reflect, col_reflect)
    }
}

fn part1(input: &str) -> String {
    let puzzles = parse(input);
    let mut cols = Vec::new();
    let mut rows = Vec::new();
    puzzles.iter().for_each(|p| {
        let (row_reflect, col_reflect) = p.find_reflection();
        if row_reflect != -1 {
            rows.push(row_reflect);
        }
        if col_reflect != -1 {
            cols.push(col_reflect);
        }
    });
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
        let result: String = part1(test_input);
        assert_eq!(result, "405".to_string());
    }
}
