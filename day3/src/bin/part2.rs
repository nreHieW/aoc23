#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Cell {
    x: i32,
    y: i32,
    value: char,
}

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> String {
    let matrix = string_to_matrix(input).unwrap();
    let num_rows = matrix.len() as i32;
    let num_cols = matrix[0].len() as i32;
    let mut score = 0;

    matrix.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, cell)| {
            let value = cell.value;
            if value == '*' {
                let numerical_neighbours = dfs(&matrix, x as i32, y as i32, num_rows, num_cols);
                let gear_ratio = gear_ratio(numerical_neighbours);
                score += gear_ratio;
            }
        });
    });
    // println!("{:?}", vals);
    return score.to_string();
}

fn gear_ratio(cells: Vec<Cell>) -> i32 {
    let mut sorted_cells = cells.clone();
    sorted_cells.sort_by(|a, b| {
        // Sort by 'y' field first
        let y_comparison = a.y.cmp(&b.y);
        if y_comparison == std::cmp::Ordering::Equal {
            // If 'y' values are equal, sort by 'x' field
            a.x.cmp(&b.x)
        } else {
            y_comparison
        }
    });

    let mut res = Vec::new();
    println!("{:?}", sorted_cells);
    let mut unique_y = sorted_cells
        .iter()
        .map(|cell| cell.y)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    unique_y.sort();
    for y in unique_y.iter() {
        let mut prev = sorted_cells
            .iter()
            .filter(|cell| cell.y == *y)
            .next()
            .unwrap()
            .x;
        let mut curr = 0;
        for cell in sorted_cells.iter() {
            if cell.y == *y {
                if (cell.x - prev <= 1) && (cell.x - prev >= 0) {
                    curr = curr * 10 + cell.value.to_digit(10).unwrap();
                } else {
                    // reset
                    res.push(curr as i32);
                    curr = cell.value.to_digit(10).unwrap();
                }
                prev = cell.x;
            }
        }
        if curr != 0 {
            res.push(curr as i32);
        }
    }
    if res.len() != 2 {
        return 0;
    } else {
        return res[0] * res[1];
    }
}

fn dfs(matrix: &Vec<Vec<Cell>>, x: i32, y: i32, num_rows: i32, num_cols: i32) -> Vec<Cell> {
    let mut stack = Vec::new();
    let mut visited = Vec::new();
    let mut res = Vec::new();
    stack.push(matrix[y as usize][x as usize]);
    while !stack.is_empty() {
        let cell = stack.pop().unwrap();
        visited.push(cell);
        if cell.value.is_digit(10) && !(res.contains(&cell)) {
            res.push(cell);
        }
        let start_x = (cell.x - 1).max(0);
        let end_x = (cell.x + 1).min(num_cols - 1);
        let start_y = (cell.y - 1).max(0);
        let end_y = (cell.y + 1).min(num_rows - 1);

        for x in [start_x, cell.x, end_x].iter() {
            // note that x and y are not numbers, but references to numbers (indices)
            for y in [start_y, cell.y, end_y].iter() {
                if *x == cell.x && *y == cell.y {
                    continue;
                }
                let neighbour = &matrix[*y as usize][*x as usize]; // need to borrow because matrix is not owned by this scop

                if !(visited.contains(neighbour)) && (neighbour.value.is_digit(10)) {
                    stack.push(*neighbour);
                }
            }
        }
    }

    return res;
}

fn string_to_matrix(input: &str) -> Result<Vec<Vec<Cell>>, &'static str> {
    let mut matrix: Vec<Vec<Cell>> = Vec::new();
    input.lines().enumerate().for_each(|(y, line)| {
        matrix.push(Vec::new());
        line.chars().enumerate().for_each(|(x, value)| {
            let cell = Cell {
                x: x as i32,
                y: y as i32,
                value: value,
            };
            matrix[y].push(cell);
        });
    });

    Ok(matrix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result: String = part2(test_input);
        assert_eq!(result, "467835".to_string());
    }
}
