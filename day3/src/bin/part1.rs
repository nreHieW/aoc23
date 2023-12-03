#[derive(Debug)]
struct Cell {
    x: i32,
    y: i32,
    value: char,
}

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    let matrix = string_to_matrix(input).unwrap();
    let num_rows = matrix.len() as i32;
    let num_cols = matrix[0].len() as i32;
    let mut vals = Vec::new();

    matrix.iter().for_each(|row| {
        let mut curr_num = 0;
        let mut is_adjacent = false; // adjacent to a symbol

        row.iter().enumerate().for_each(|(x, cell)| {
            let value = cell.value;
            if value.is_digit(10) {
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
                        let neighbour = &matrix[*y as usize][*x as usize]; // need to borrow because matrix is not owned by this scope
                        if !(neighbour.value.is_digit(10)) && (neighbour.value != '.') {
                            is_adjacent = true;
                        }
                    }
                }
                curr_num = curr_num * 10 + value.to_digit(10).unwrap();
            }
            if (x == row.len() - 1) || !(value.is_digit(10)) {
                if is_adjacent {
                    vals.push(curr_num);
                }
                is_adjacent = false;
                curr_num = 0;
            }
        });
    });
    // println!("{:?}", vals);
    return vals.iter().sum::<u32>().to_string();
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
        let result: String = part1(test_input);
        assert_eq!(result, "4361".to_string());
    }
}
